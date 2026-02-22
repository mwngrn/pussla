#!/usr/bin/env python3
"""Validate Pussla planning data for schema, over-allocation, cross-references, and PII leakage."""

from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

import yaml


ISO_WEEK_RE = re.compile(r"^\d{4}-W(0[1-9]|[1-4][0-9]|5[0-3])$")
EMAIL_RE = re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}")
PHONE_RE = re.compile(r"\+?\d[\d\s().-]{7,}\d")


def read_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def parse_frontmatter(path: Path) -> tuple[dict[str, Any], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError("missing YAML frontmatter start delimiter '---'")
    parts = text.split("\n---\n", 1)
    if len(parts) != 2:
        raise ValueError("missing YAML frontmatter end delimiter '---'")
    frontmatter_text = parts[0][4:]
    body = parts[1]
    data = yaml.safe_load(frontmatter_text) or {}
    if not isinstance(data, dict):
        raise ValueError("frontmatter must parse to a YAML object")
    return data, body


def load_real_names(identity_dir: Path) -> list[str]:
    names: list[str] = []
    if not identity_dir.exists():
        return names
    for path in sorted(identity_dir.glob("*.md")):
        try:
            frontmatter, _ = parse_frontmatter(path)
        except Exception:
            continue
        value = frontmatter.get("real_name")
        if isinstance(value, str) and value.strip():
            names.append(value.strip())
    return names


def validate_allocations(allocations_dir: Path) -> tuple[list[str], dict[str, dict[str, int]], set[str]]:
    errors: list[str] = []
    totals: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    referenced_projects: set[str] = set()

    for path in sorted(allocations_dir.glob("*.yaml")):
        try:
            data = read_yaml(path)
        except Exception as exc:
            errors.append(f"{path}: failed to parse YAML: {exc}")
            continue

        if not isinstance(data, dict):
            errors.append(f"{path}: root must be a YAML object")
            continue

        alias = data.get("alias")
        entries = data.get("allocations")

        if not isinstance(alias, str) or not alias.strip():
            errors.append(f"{path}: 'alias' must be a non-empty string")
            continue
        if path.stem != alias:
            errors.append(f"{path}: filename stem must match alias ('{alias}')")

        if not isinstance(entries, list):
            errors.append(f"{path}: 'allocations' must be a list")
            continue

        for idx, item in enumerate(entries):
            location = f"{path} allocations[{idx}]"
            if not isinstance(item, dict):
                errors.append(f"{location}: entry must be an object")
                continue

            project = item.get("project")
            weeks = item.get("weeks")
            load = item.get("load")

            if not isinstance(project, str) or not project.strip():
                errors.append(f"{location}: 'project' must be a non-empty string")
            else:
                referenced_projects.add(project)

            if not isinstance(weeks, list) or not weeks:
                errors.append(f"{location}: 'weeks' must be a non-empty list")
                weeks = []
            if not isinstance(load, int):
                errors.append(f"{location}: 'load' must be an integer")
                continue
            if load < 0 or load > 100:
                errors.append(f"{location}: 'load' must be between 0 and 100")

            for week in weeks:
                if not isinstance(week, str) or not ISO_WEEK_RE.match(week):
                    errors.append(f"{location}: invalid ISO week '{week}' (expected YYYY-Www)")
                    continue
                totals[alias][week] += load

    for alias, by_week in sorted(totals.items()):
        for week, total in sorted(by_week.items()):
            if total > 100:
                errors.append(
                    f"over-allocation: alias '{alias}' has total load {total} in week {week} (>100)"
                )

    return errors, totals, referenced_projects


def validate_projects(projects_dir: Path) -> tuple[list[str], set[str]]:
    errors: list[str] = []
    referenced_aliases: set[str] = set()
    required = {"project_id", "name", "owner_alias", "start_week", "end_week", "status", "team_aliases"}

    for path in sorted(projects_dir.glob("*.md")):
        try:
            frontmatter, body = parse_frontmatter(path)
        except Exception as exc:
            errors.append(f"{path}: {exc}")
            continue

        missing = [k for k in sorted(required) if k not in frontmatter]
        if missing:
            errors.append(f"{path}: missing required frontmatter field(s): {', '.join(missing)}")
            continue

        if not isinstance(frontmatter["project_id"], str) or not frontmatter["project_id"].strip():
            errors.append(f"{path}: 'project_id' must be a non-empty string")
        if not isinstance(frontmatter["name"], str) or not frontmatter["name"].strip():
            errors.append(f"{path}: 'name' must be a non-empty string")
        
        owner = frontmatter.get("owner_alias")
        if not isinstance(owner, str) or not owner.strip():
            errors.append(f"{path}: 'owner_alias' must be a non-empty string")
        else:
            referenced_aliases.add(owner)

        for field in ("start_week", "end_week"):
            value = frontmatter.get(field)
            if not isinstance(value, str) or not ISO_WEEK_RE.match(value):
                errors.append(f"{path}: '{field}' must be ISO week string (YYYY-Www)")

        if not isinstance(frontmatter["status"], str) or not frontmatter["status"].strip():
            errors.append(f"{path}: 'status' must be a non-empty string")
        
        team = frontmatter.get("team_aliases")
        if not isinstance(team, list):
            errors.append(f"{path}: 'team_aliases' must be a list")
        else:
            for alias in team:
                if isinstance(alias, str):
                    referenced_aliases.add(alias)

        if not body.strip():
            errors.append(f"{path}: markdown body must not be empty")

    return errors, referenced_aliases


def check_pii_leaks(public_files: list[Path], real_names: list[str]) -> list[str]:
    errors: list[str] = []
    lowered_names = [name.lower() for name in real_names]

    for path in public_files:
        text = path.read_text(encoding="utf-8")
        lower = text.lower()

        for name in lowered_names:
            if name and name in lower:
                errors.append(f"{path}: potential PII leak, contains identity real_name '{name}'")

        if EMAIL_RE.search(text):
            errors.append(f"{path}: potential PII leak, contains email-like text")
        if PHONE_RE.search(text):
            errors.append(f"{path}: potential PII leak, contains phone-like text")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate Pussla planning dataset")
    parser.add_argument("--planning-dir", default="tst-data/planning", help="Path containing allocations/ and projects/")
    parser.add_argument("--identity-dir", default="tst-data/identity", help="Path containing identity markdown files")
    args = parser.parse_args()

    planning_dir = Path(args.planning_dir)
    identity_dir = Path(args.identity_dir)
    allocations_dir = planning_dir / "allocations"
    projects_dir = planning_dir / "projects"

    errors: list[str] = []
    if not allocations_dir.exists():
        errors.append(f"missing directory: {allocations_dir}")
    if not projects_dir.exists():
        errors.append(f"missing directory: {projects_dir}")
    if errors:
        for err in errors:
            print(f"ERROR: {err}")
        return 1

    alloc_errors, totals, ref_projects = validate_allocations(allocations_dir)
    project_errors, ref_aliases = validate_projects(projects_dir)
    
    known_aliases = {p.stem for p in allocations_dir.glob("*.yaml")}
    known_projects = {p.stem for p in projects_dir.glob("*.md")}

    cross_errors: list[str] = []
    for proj in sorted(ref_projects):
        if proj not in known_projects:
            cross_errors.append(f"cross-reference: project '{proj}' referenced in allocations but not found in projects/")
    
    for alias in sorted(ref_aliases):
        if alias not in known_aliases:
            cross_errors.append(f"cross-reference: alias '{alias}' referenced in projects but not found in allocations/")

    real_names = load_real_names(identity_dir)
    pii_errors = check_pii_leaks(
        [*sorted(allocations_dir.glob("*.yaml")), *sorted(projects_dir.glob("*.md"))],
        real_names,
    )

    all_errors = [*alloc_errors, *project_errors, *cross_errors, *pii_errors]
    if all_errors:
        for err in all_errors:
            print(f"ERROR: {err}")
        print(f"\nValidation failed with {len(all_errors)} error(s).")
        return 1

    print("Validation passed.")
    print(f"- allocations files: {len(known_aliases)}")
    print(f"- project files: {len(known_projects)}")
    print(f"- identity files scanned: {len(list(identity_dir.glob('*.md')))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
