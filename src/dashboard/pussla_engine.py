from __future__ import annotations

import json
import re
import argparse
from collections import defaultdict
from datetime import date, datetime
from tempfile import NamedTemporaryFile
from pathlib import Path
from typing import Any

import yaml

ISO_WEEK_RE = re.compile(r"^(\d{4})-W(0[1-9]|[1-4][0-9]|5[0-3])$")
DEFAULT_CAPACITY_HOURS = 40.0


def _parse_iso_week(value: str) -> tuple[int, int] | None:
    m = ISO_WEEK_RE.match(value)
    if not m:
        return None
    return int(m.group(1)), int(m.group(2))


def _normalize_week(value: str) -> str | None:
    parsed = _parse_iso_week(value)
    if parsed is None:
        return None
    return f"{parsed[0]:04d}-W{parsed[1]:02d}"


def _week_sort_key(value: str) -> tuple[int, int]:
    parsed = _parse_iso_week(value)
    return parsed if parsed is not None else (9999, 53)


def _to_hours_from_load(load: int, capacity_hours: float) -> float:
    return round((load / 100.0) * capacity_hours, 1)


def _to_load_from_hours(hours: float, capacity_hours: float) -> int:
    if capacity_hours <= 0:
        return 0
    return int(round((hours / capacity_hours) * 100))


def _resolve_planning_dir(data_dir: Path, planning_override: str | None) -> Path:
    if planning_override:
        return Path(planning_override)

    preferred = data_dir / "planning"
    legacy = data_dir / "planing"
    if preferred.exists() or not legacy.exists():
        return preferred
    return legacy


def _parse_frontmatter(path: Path) -> tuple[dict[str, Any], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return {}, text

    parts = text.split("\n---\n", 1)
    if len(parts) != 2:
        return {}, text

    frontmatter = yaml.safe_load(parts[0][4:]) or {}
    if not isinstance(frontmatter, dict):
        frontmatter = {}

    return frontmatter, parts[1]


def _collect_identities(identity_dir: Path) -> dict[str, dict[str, str | None]]:
    identities: dict[str, dict[str, str | None]] = {}
    if not identity_dir.exists():
        return identities

    for path in sorted(identity_dir.glob("*.md")):
        frontmatter, _ = _parse_frontmatter(path)
        alias = frontmatter.get("alias")
        if not isinstance(alias, str) or not alias.strip():
            continue

        real_name = frontmatter.get("real_name")
        role = frontmatter.get("role")
        identities[alias] = {
            "real_name": real_name if isinstance(real_name, str) else None,
            "role": role if isinstance(role, str) and role.strip() else "Consultant",
        }

    return identities


def _collect_project_context(projects_dir: Path) -> dict[str, dict[str, Any]]:
    contexts: dict[str, dict[str, Any]] = {}
    if not projects_dir.exists():
        return contexts

    for path in sorted(projects_dir.glob("*.md")):
        frontmatter, body = _parse_frontmatter(path)
        name = frontmatter.get("name")
        if not isinstance(name, str) or not name.strip():
            name = path.stem

        summary = ""
        for line in body.splitlines():
            if line.strip():
                summary = line.strip()
                break

        status = frontmatter.get("status")
        owner_alias = frontmatter.get("owner_alias")
        start_week = frontmatter.get("start_week")
        end_week = frontmatter.get("end_week")
        start_week_override = frontmatter.get("start_week_override")
        end_week_override = frontmatter.get("end_week_override")
        hourly_rate = frontmatter.get("hourly_rate")
        milestones = frontmatter.get("milestones")
        if not isinstance(milestones, list):
            milestones = []
        normalized_milestones: list[dict[str, Any]] = []
        for idx, ms in enumerate(milestones):
            if not isinstance(ms, dict):
                continue
            title = ms.get("title")
            milestone_date = ms.get("date")
            if not isinstance(title, str) or not title.strip():
                continue
            if isinstance(milestone_date, (date, datetime)):
                date_value = milestone_date.isoformat()[:10]
            elif isinstance(milestone_date, str):
                date_value = milestone_date
            else:
                continue
            if not re.match(r"^\d{4}-\d{2}-\d{2}$", date_value):
                continue
            normalized_milestones.append(
                {
                    "id": ms.get("id") if isinstance(ms.get("id"), str) else f"ms-{idx+1}",
                    "title": title.strip(),
                    "date": date_value,
                }
            )
        normalized_milestones.sort(key=lambda m: m["date"])
        contexts[name] = {
            "project_id": frontmatter.get("project_id") if isinstance(frontmatter.get("project_id"), str) else path.stem,
            "status": status if isinstance(status, str) else None,
            "owner_alias": owner_alias if isinstance(owner_alias, str) else None,
            "start_week": start_week if isinstance(start_week, str) else None,
            "end_week": end_week if isinstance(end_week, str) else None,
            "start_week_override": start_week_override if isinstance(start_week_override, str) else None,
            "end_week_override": end_week_override if isinstance(end_week_override, str) else None,
            "hourly_rate": float(hourly_rate) if isinstance(hourly_rate, (int, float)) else None,
            "milestones": normalized_milestones,
            "summary": summary,
            "source_file": path.name,
        }

    return contexts


def update_week_allocations(
    planning_dir: str | Path,
    alias: str,
    week: str,
    allocations: list[dict[str, Any]],
) -> dict[str, Any]:
    if not isinstance(alias, str) or not alias.strip():
        raise ValueError("alias must be a non-empty string")

    normalized_week = _normalize_week(week)
    if normalized_week is None:
        raise ValueError("week must be in YYYY-Www format")

    if not isinstance(allocations, list):
        raise ValueError("allocations must be a list")

    normalized_entries: list[dict[str, Any]] = []
    merged_by_project: dict[tuple[str, str], dict[str, float | int | str]] = {}
    for entry in allocations:
        if not isinstance(entry, dict):
            raise ValueError("each allocation entry must be an object")

        project = entry.get("project")
        load = entry.get("load")
        planned_hours = entry.get("planned_hours")
        capacity_hours = entry.get("capacity_hours")
        state = entry.get("state")
        if not isinstance(project, str) or not project.strip():
            raise ValueError("allocation project must be a non-empty string")
        if state is None:
            state_value = "committed"
        elif isinstance(state, str) and state.strip().lower() in {"tentative", "committed"}:
            state_value = state.strip().lower()
        else:
            raise ValueError("allocation state must be 'tentative' or 'committed'")
        if capacity_hours is None:
            capacity = DEFAULT_CAPACITY_HOURS
        elif isinstance(capacity_hours, (int, float)):
            capacity = float(capacity_hours)
        else:
            raise ValueError("allocation capacity_hours must be a number")
        if capacity <= 0:
            raise ValueError("allocation capacity_hours must be greater than 0")

        project = project.strip()
        if planned_hours is not None:
            if not isinstance(planned_hours, (int, float)):
                raise ValueError("allocation planned_hours must be a number")
            hours = float(planned_hours)
            if hours < 0:
                raise ValueError("allocation planned_hours cannot be negative")
            load_value = _to_load_from_hours(hours, capacity)
        else:
            if not isinstance(load, int):
                raise ValueError("allocation load must be an integer")
            if load < 0:
                raise ValueError("allocation load cannot be negative")
            load_value = load
            hours = _to_hours_from_load(load_value, capacity)

        key = (project, state_value)
        current = merged_by_project.get(key)
        if current is None:
            merged_by_project[key] = {
                "planned_hours": hours,
                "capacity_hours": capacity,
                "load": load_value,
                "state": state_value,
            }
        else:
            total_hours = float(current["planned_hours"]) + hours
            merged_capacity = float(current["capacity_hours"])
            merged_load = _to_load_from_hours(total_hours, merged_capacity)
            merged_by_project[key] = {
                "planned_hours": round(total_hours, 1),
                "capacity_hours": merged_capacity,
                "load": merged_load,
                "state": state_value,
            }

    for (project, _state), values in merged_by_project.items():
        planned_hours = float(values["planned_hours"])
        capacity_hours = float(values["capacity_hours"])
        load = int(values["load"])
        state = str(values["state"])
        if planned_hours <= 0:
            continue
        normalized_entries.append(
            {
                "project": project,
                "planned_hours": planned_hours,
                "capacity_hours": capacity_hours,
                "load": load,
                "state": state,
            }
        )

    planning_path = Path(planning_dir)
    alloc_file = planning_path / "allocations" / f"{alias}.yaml"
    if not alloc_file.exists():
        raise FileNotFoundError(f"allocation file not found for alias '{alias}'")

    try:
        data = yaml.safe_load(alloc_file.read_text(encoding="utf-8")) or {}
    except Exception as exc:
        raise ValueError(f"failed to parse allocation file for '{alias}'") from exc

    if not isinstance(data, dict):
        raise ValueError(f"allocation file for '{alias}' must contain a YAML object")

    file_alias = data.get("alias")
    if not isinstance(file_alias, str) or not file_alias.strip():
        data["alias"] = alias
    elif file_alias != alias:
        raise ValueError(f"allocation file alias mismatch: expected '{alias}', found '{file_alias}'")

    raw_allocations = data.get("allocations")
    if raw_allocations is None:
        raw_allocations = []
    if not isinstance(raw_allocations, list):
        raise ValueError("allocation file field 'allocations' must be a list")

    rebuilt: list[dict[str, Any]] = []
    for entry in raw_allocations:
        if not isinstance(entry, dict):
            continue
        project = entry.get("project")
        weeks = entry.get("weeks")
        load = entry.get("load")
        planned_hours = entry.get("planned_hours")
        capacity_hours = entry.get("capacity_hours")
        state = entry.get("state")
        if not isinstance(project, str) or not isinstance(weeks, list):
            continue
        if state is None:
            state_value = "committed"
        elif isinstance(state, str) and state.strip().lower() in {"tentative", "committed"}:
            state_value = state.strip().lower()
        else:
            continue
        if capacity_hours is None:
            capacity = DEFAULT_CAPACITY_HOURS
        elif isinstance(capacity_hours, (int, float)):
            capacity = float(capacity_hours)
        else:
            continue
        if capacity <= 0:
            continue
        if isinstance(planned_hours, (int, float)):
            hours = float(planned_hours)
            load_value = _to_load_from_hours(hours, capacity)
        elif isinstance(load, int):
            load_value = load
            hours = _to_hours_from_load(load_value, capacity)
        else:
            continue

        kept_weeks: list[str] = []
        for raw_week in weeks:
            if not isinstance(raw_week, str):
                continue
            normalized = _normalize_week(raw_week)
            if normalized is None or normalized == normalized_week:
                continue
            kept_weeks.append(normalized)

        if not kept_weeks:
            continue

        unique_sorted_weeks = sorted(set(kept_weeks), key=_week_sort_key)
        rebuilt.append(
            {
                "project": project,
                "weeks": unique_sorted_weeks,
                "planned_hours": round(hours, 1),
                "capacity_hours": capacity,
                "load": load_value,
                "state": state_value,
            }
        )

    for entry in normalized_entries:
        project = entry["project"]
        load = entry["load"]
        planned_hours = entry["planned_hours"]
        capacity_hours = entry["capacity_hours"]
        state = entry["state"]
        existing = None
        for candidate in rebuilt:
            if (
                candidate.get("project") == project
                and candidate.get("load") == load
                and candidate.get("planned_hours") == planned_hours
                and candidate.get("capacity_hours") == capacity_hours
                and candidate.get("state") == state
            ):
                existing = candidate
                break

        if existing is None:
            rebuilt.append(
                {
                    "project": project,
                    "weeks": [normalized_week],
                    "planned_hours": planned_hours,
                    "capacity_hours": capacity_hours,
                    "load": load,
                    "state": state,
                }
            )
        else:
            weeks = [w for w in existing.get("weeks", []) if isinstance(w, str)]
            if normalized_week not in weeks:
                weeks.append(normalized_week)
            existing["weeks"] = sorted(set(weeks), key=_week_sort_key)

    data["allocations"] = rebuilt

    alloc_file.parent.mkdir(parents=True, exist_ok=True)
    with NamedTemporaryFile("w", encoding="utf-8", dir=alloc_file.parent, delete=False) as tmp:
        yaml.safe_dump(data, tmp, sort_keys=False, allow_unicode=True)
        temp_path = Path(tmp.name)

    temp_path.replace(alloc_file)

    total_load = sum(entry["load"] for entry in normalized_entries)
    total_planned_hours = round(sum(float(entry["planned_hours"]) for entry in normalized_entries), 1)
    return {
        "alias": alias,
        "week": normalized_week,
        "projects_count": len(normalized_entries),
        "total_load": total_load,
        "total_planned_hours": total_planned_hours,
        "capacity_hours": DEFAULT_CAPACITY_HOURS,
    }


def update_project_metadata(
    planning_dir: str | Path,
    project: str,
    updates: dict[str, Any],
) -> dict[str, Any]:
    if not isinstance(project, str) or not project.strip():
        raise ValueError("project must be a non-empty string")
    if not isinstance(updates, dict):
        raise ValueError("updates must be an object")

    projects_dir = Path(planning_dir) / "projects"
    if not projects_dir.exists():
        raise FileNotFoundError("projects directory not found")

    target_path: Path | None = None
    frontmatter: dict[str, Any] = {}
    body = ""
    for path in sorted(projects_dir.glob("*.md")):
        fm, md_body = _parse_frontmatter(path)
        name = fm.get("name")
        if (isinstance(name, str) and name == project) or path.stem == project:
            target_path = path
            frontmatter = fm
            body = md_body
            break
    if target_path is None:
        raise FileNotFoundError(f"project file not found for '{project}'")

    allowed = {"hourly_rate", "milestones", "start_week_override", "end_week_override"}
    for key in updates:
        if key not in allowed:
            raise ValueError(f"unsupported project update field: {key}")

    if "hourly_rate" in updates:
        rate = updates["hourly_rate"]
        if rate is None:
            frontmatter.pop("hourly_rate", None)
        elif isinstance(rate, (int, float)) and float(rate) >= 0:
            frontmatter["hourly_rate"] = round(float(rate), 2)
        else:
            raise ValueError("hourly_rate must be a non-negative number or null")

    for key in ("start_week_override", "end_week_override"):
        if key in updates:
            value = updates[key]
            if value in (None, ""):
                frontmatter.pop(key, None)
            elif isinstance(value, str) and _normalize_week(value) is not None:
                frontmatter[key] = _normalize_week(value)
            else:
                raise ValueError(f"{key} must be an ISO week (YYYY-Www) or null")

    if "milestones" in updates:
        milestones = updates["milestones"]
        if not isinstance(milestones, list):
            raise ValueError("milestones must be a list")
        normalized_milestones: list[dict[str, str]] = []
        for idx, ms in enumerate(milestones):
            if not isinstance(ms, dict):
                raise ValueError("each milestone must be an object")
            title = ms.get("title")
            date = ms.get("date")
            if not isinstance(title, str) or not title.strip():
                raise ValueError("milestone title must be a non-empty string")
            if not isinstance(date, str) or not re.match(r"^\d{4}-\d{2}-\d{2}$", date):
                raise ValueError("milestone date must be in YYYY-MM-DD format")
            normalized_milestones.append(
                {
                    "id": ms.get("id") if isinstance(ms.get("id"), str) else f"ms-{idx+1}",
                    "title": title.strip(),
                    "date": date,
                }
            )
        normalized_milestones.sort(key=lambda m: m["date"])
        frontmatter["milestones"] = normalized_milestones

    rendered = f"---\n{yaml.safe_dump(frontmatter, sort_keys=False, allow_unicode=True).strip()}\n---\n{body}"
    with NamedTemporaryFile("w", encoding="utf-8", dir=target_path.parent, delete=False) as tmp:
        tmp.write(rendered)
        temp_path = Path(tmp.name)
    temp_path.replace(target_path)

    return {"project": project, "file": target_path.name}


def build_dashboard_data(
    planning_dir: str | Path,
    identity_dir: str | Path,
    include_pii: bool = True,
) -> dict[str, Any]:
    planning_path = Path(planning_dir)
    identity_path = Path(identity_dir)
    allocations_dir = planning_path / "allocations"
    projects_dir = planning_path / "projects"

    identities = _collect_identities(identity_path)
    project_context = _collect_project_context(projects_dir)

    users_by_alias: dict[str, dict[str, Any]] = {}
    seen_weeks: set[str] = set()
    raw_allocations: list[dict[str, Any]] = []
    project_week_bounds: dict[str, dict[str, str]] = {}

    for alloc_file in sorted(allocations_dir.glob("*.yaml")):
        try:
            data = yaml.safe_load(alloc_file.read_text(encoding="utf-8")) or {}
        except Exception:
            continue

        if not isinstance(data, dict):
            continue

        alias = data.get("alias")
        entries = data.get("allocations")
        if not isinstance(alias, str) or not isinstance(entries, list):
            continue

        identity = identities.get(alias, {})
        user = users_by_alias.setdefault(
            alias,
            {
                "alias": alias,
                "real_name": identity.get("real_name") if include_pii else None,
                "role": identity.get("role") or "Consultant",
                "weekly": defaultdict(
                    lambda: {
                        "total_load": 0.0,
                        "total_planned_hours": 0.0,
                        "capacity_hours": DEFAULT_CAPACITY_HOURS,
                        "projects": [],
                    }
                ),
            },
        )

        for entry in entries:
            if not isinstance(entry, dict):
                continue
            project = entry.get("project")
            weeks = entry.get("weeks")
            load = entry.get("load")
            planned_hours = entry.get("planned_hours")
            capacity_hours = entry.get("capacity_hours")
            state = entry.get("state")
            if not isinstance(project, str) or not isinstance(weeks, list):
                continue
            if state is None:
                state_value = "committed"
            elif isinstance(state, str) and state.strip().lower() in {"tentative", "committed"}:
                state_value = state.strip().lower()
            else:
                continue
            if capacity_hours is None:
                capacity = DEFAULT_CAPACITY_HOURS
            elif isinstance(capacity_hours, (int, float)):
                capacity = float(capacity_hours)
            else:
                continue
            if capacity <= 0:
                continue
            if isinstance(planned_hours, (int, float)):
                hours = float(planned_hours)
                load_value = _to_load_from_hours(hours, capacity)
            elif isinstance(load, int):
                load_value = load
                hours = _to_hours_from_load(load_value, capacity)
            else:
                continue

            for week in weeks:
                if not isinstance(week, str):
                    continue
                normalized = _normalize_week(week)
                if normalized is None:
                    continue
                seen_weeks.add(normalized)
                bounds = project_week_bounds.setdefault(
                    project, {"min_week": normalized, "max_week": normalized}
                )
                if normalized < bounds["min_week"]:
                    bounds["min_week"] = normalized
                if normalized > bounds["max_week"]:
                    bounds["max_week"] = normalized

                bucket = user["weekly"][normalized]
                bucket["total_planned_hours"] += hours
                if capacity > 0:
                    bucket["capacity_hours"] = capacity
                bucket["total_load"] = round(
                    (bucket["total_planned_hours"] / bucket["capacity_hours"]) * 100, 1
                )
                bucket["projects"].append(
                    {
                        "project": project,
                        "load": load_value,
                        "planned_hours": round(hours, 1),
                        "capacity_hours": capacity,
                        "state": state_value,
                        "context": project_context.get(project, {}),
                    }
                )

                raw_allocations.append(
                    {
                        "alias": alias,
                        "week": normalized,
                        "project": project,
                        "load": load_value,
                        "planned_hours": round(hours, 1),
                        "capacity_hours": capacity,
                        "state": state_value,
                    }
                )

    sorted_weeks = sorted(seen_weeks, key=_week_sort_key)

    users: list[dict[str, Any]] = []
    for alias in sorted(users_by_alias):
        user = users_by_alias[alias]
        weekly_stats = []
        for week in sorted_weeks:
            bucket = user["weekly"].get(
                week,
                {
                    "total_load": 0.0,
                    "total_planned_hours": 0.0,
                    "capacity_hours": DEFAULT_CAPACITY_HOURS,
                    "projects": [],
                },
            )
            weekly_stats.append(
                {
                    "week": week,
                    "total_load": round(float(bucket["total_load"]), 1),
                    "total_planned_hours": round(float(bucket["total_planned_hours"]), 1),
                    "capacity_hours": round(float(bucket["capacity_hours"]), 1),
                    "projects": bucket["projects"],
                }
            )

        real_name = user["real_name"]
        users.append(
            {
                "alias": alias,
                "real_name": real_name,
                "display_name": real_name if isinstance(real_name, str) and real_name.strip() else alias,
                "role": user["role"],
                "weekly_stats": weekly_stats,
            }
        )

    total_slots = 0
    total_load_points = 0
    overbooked_slots = 0
    for user in users:
        for slot in user["weekly_stats"]:
            total_slots += 1
            total_load_points += slot["total_load"]
            if slot["total_load"] > 100:
                overbooked_slots += 1

    avg_util = round(total_load_points / total_slots, 1) if total_slots else 0.0

    projects: list[dict[str, Any]] = []
    for project_name in sorted(project_context):
        ctx = dict(project_context[project_name])
        bounds = project_week_bounds.get(project_name, {})
        derived_start = bounds.get("min_week")
        derived_end = bounds.get("max_week")
        resolved_start = ctx.get("start_week_override") or derived_start or ctx.get("start_week")
        resolved_end = ctx.get("end_week_override") or derived_end or ctx.get("end_week")
        projects.append(
            {
                "name": project_name,
                **ctx,
                "derived_start_week": derived_start,
                "derived_end_week": derived_end,
                "resolved_start_week": resolved_start,
                "resolved_end_week": resolved_end,
            }
        )

    return {
        "generated_at": datetime.now().isoformat(timespec="seconds"),
        "weeks": sorted_weeks,
        "users": users,
        "projects": projects,
        "metrics": {
            "users_count": len(users),
            "average_utilization": avg_util,
            "overbooked_slots": overbooked_slots,
        },
        "raw_allocations": raw_allocations,
    }


def write_dashboard_json(
    output_file: str | Path = "pussla_data.json",
    planning_dir: str | Path = "tst-data/planning",
    identity_dir: str | Path = "tst-data/identity",
    include_pii: bool = True,
) -> Path:
    data = build_dashboard_data(
        planning_dir=planning_dir,
        identity_dir=identity_dir,
        include_pii=include_pii,
    )
    output_path = Path(output_file)
    output_path.write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8")
    return output_path


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Build dashboard JSON from Pussla data")
    parser.add_argument("--data-dir", default="tst-data", help="Base folder containing planning/ (or legacy planing/) and identity/")
    parser.add_argument("--planning-dir", default=None, help="Override planning folder (contains allocations/ and projects/)")
    parser.add_argument("--identity-dir", default=None, help="Override identity folder")
    parser.add_argument("--output-file", default="pussla_data.json")
    parser.add_argument("--no-pii", action="store_true", help="Exclude real names from output")
    args = parser.parse_args()

    data_dir = Path(args.data_dir)
    planning_dir = _resolve_planning_dir(data_dir, args.planning_dir)
    identity_dir = Path(args.identity_dir) if args.identity_dir else data_dir / "identity"

    output = write_dashboard_json(
        output_file=args.output_file,
        planning_dir=planning_dir,
        identity_dir=identity_dir,
        include_pii=not args.no_pii,
    )
    print(f"Wrote {output}")
