from __future__ import annotations

import json
import re
import argparse
from collections import defaultdict
from datetime import datetime
from pathlib import Path
from typing import Any

import yaml

ISO_WEEK_RE = re.compile(r"^(\d{4})-W(0[1-9]|[1-4][0-9]|5[0-3])$")


def _parse_iso_week(value: str) -> tuple[int, int] | None:
    m = ISO_WEEK_RE.match(value)
    if not m:
        return None
    return int(m.group(1)), int(m.group(2))


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


def _collect_project_context(projects_dir: Path) -> dict[str, dict[str, str | None]]:
    contexts: dict[str, dict[str, str | None]] = {}
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
        contexts[name] = {
            "status": status if isinstance(status, str) else None,
            "owner_alias": owner_alias if isinstance(owner_alias, str) else None,
            "summary": summary,
        }

    return contexts


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
                "weekly": defaultdict(lambda: {"total_load": 0, "projects": []}),
            },
        )

        for entry in entries:
            if not isinstance(entry, dict):
                continue
            project = entry.get("project")
            weeks = entry.get("weeks")
            load = entry.get("load")
            if not isinstance(project, str) or not isinstance(weeks, list) or not isinstance(load, int):
                continue

            for week in weeks:
                if not isinstance(week, str):
                    continue
                parsed = _parse_iso_week(week)
                if parsed is None:
                    continue

                normalized = f"{parsed[0]:04d}-W{parsed[1]:02d}"
                seen_weeks.add(normalized)

                bucket = user["weekly"][normalized]
                bucket["total_load"] += load
                bucket["projects"].append(
                    {
                        "project": project,
                        "load": load,
                        "context": project_context.get(project, {}),
                    }
                )

                raw_allocations.append(
                    {
                        "alias": alias,
                        "week": normalized,
                        "project": project,
                        "load": load,
                    }
                )

    sorted_weeks = sorted(seen_weeks, key=lambda w: _parse_iso_week(w) or (9999, 53))

    users: list[dict[str, Any]] = []
    for alias in sorted(users_by_alias):
        user = users_by_alias[alias]
        weekly_stats = []
        for week in sorted_weeks:
            bucket = user["weekly"].get(week, {"total_load": 0, "projects": []})
            weekly_stats.append(
                {
                    "week": week,
                    "total_load": bucket["total_load"],
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

    return {
        "generated_at": datetime.now().isoformat(timespec="seconds"),
        "weeks": sorted_weeks,
        "users": users,
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
