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
ROLE_ID_RE = re.compile(r"^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*$")
EMAIL_RE = re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}")
PHONE_RE = re.compile(r"\+?\d[\d \t().-]{7,}\d")
DEFAULT_CAPACITY_HOURS = 40.0
ROLE_REQUIRED_FIELDS = {"role_id", "name"}
SKILLS_REQUIRED_FIELDS = {"canonical_skills"}


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


def normalize_skill(value: str) -> str:
    return value.strip().lower()


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


def validate_roles(roles_dir: Path) -> tuple[list[str], list[str], dict[str, str]]:
    errors: list[str] = []
    warnings: list[str] = []
    role_names: dict[str, str] = {}

    for path in sorted(roles_dir.glob("*.md")):
        try:
            frontmatter, _body = parse_frontmatter(path)
        except Exception as exc:
            errors.append(f"{path}: {exc}")
            continue

        missing = [k for k in sorted(ROLE_REQUIRED_FIELDS) if k not in frontmatter]
        if missing:
            errors.append(f"{path}: missing required frontmatter field(s): {', '.join(missing)}")
            continue

        role_id = frontmatter.get("role_id")
        name = frontmatter.get("name")
        if not isinstance(role_id, str) or not role_id.strip():
            errors.append(f"{path}: 'role_id' must be a non-empty string")
            continue
        if not ROLE_ID_RE.match(role_id):
            errors.append(
                f"{path}: 'role_id' must match {ROLE_ID_RE.pattern}"
            )
            continue
        if role_id in role_names:
            errors.append(f"{path}: duplicate role_id '{role_id}'")
            continue

        if not isinstance(name, str) or not name.strip():
            errors.append(f"{path}: 'name' must be a non-empty string")
            continue

        for extra_key in sorted(k for k in frontmatter.keys() if k not in ROLE_REQUIRED_FIELDS):
            warnings.append(f"{path}: unknown role frontmatter field '{extra_key}' (allowed, but ignored by schema)")

        role_names[role_id] = name.strip()

    return errors, warnings, role_names


def validate_skills_catalog(skills_path: Path) -> tuple[list[str], list[str], set[str], dict[str, str]]:
    errors: list[str] = []
    warnings: list[str] = []
    canonical: set[str] = set()
    synonyms: dict[str, str] = {}

    try:
        frontmatter, _body = parse_frontmatter(skills_path)
    except Exception as exc:
        return [f"{skills_path}: {exc}"], warnings, canonical, synonyms

    missing = [k for k in sorted(SKILLS_REQUIRED_FIELDS) if k not in frontmatter]
    if missing:
        errors.append(f"{skills_path}: missing required frontmatter field(s): {', '.join(missing)}")
        return errors, warnings, canonical, synonyms

    canonical_skills = frontmatter.get("canonical_skills")
    if not isinstance(canonical_skills, list):
        errors.append(f"{skills_path}: 'canonical_skills' must be a list")
        return errors, warnings, canonical, synonyms

    for idx, skill in enumerate(canonical_skills):
        if not isinstance(skill, str) or not skill.strip():
            errors.append(f"{skills_path}: canonical_skills[{idx}] must be a non-empty string")
            continue
        canonical.add(normalize_skill(skill))

    raw_synonyms = frontmatter.get("synonyms", {})
    if raw_synonyms is None:
        raw_synonyms = {}
    if not isinstance(raw_synonyms, dict):
        errors.append(f"{skills_path}: 'synonyms' must be an object/map when provided")
        return errors, warnings, canonical, synonyms

    for raw_key, raw_target in raw_synonyms.items():
        if not isinstance(raw_key, str) or not raw_key.strip():
            errors.append(f"{skills_path}: synonym key '{raw_key}' must be a non-empty string")
            continue
        if not isinstance(raw_target, str) or not raw_target.strip():
            errors.append(f"{skills_path}: synonym target for '{raw_key}' must be a non-empty string")
            continue
        key = normalize_skill(raw_key)
        target = normalize_skill(raw_target)
        synonyms[key] = target
        if canonical and target not in canonical:
            warnings.append(
                f"{skills_path}: synonym '{raw_key}' points to non-canonical target '{raw_target}'"
            )

    return errors, warnings, canonical, synonyms


def validate_people(
    people_dir: Path,
    known_roles: set[str],
    canonical_skills: set[str],
    skill_synonyms: dict[str, str],
) -> tuple[list[str], list[str], dict[str, dict[str, float]], set[str], set[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    totals: dict[str, dict[str, float]] = defaultdict(lambda: defaultdict(float))
    capacities: dict[str, dict[str, float]] = defaultdict(dict)
    referenced_projects: set[str] = set()
    known_aliases: set[str] = set()

    for path in sorted(people_dir.glob("*.md")):
        try:
            frontmatter, _body = parse_frontmatter(path)
        except Exception as exc:
            errors.append(f"{path}: {exc}")
            continue

        alias = frontmatter.get("alias")
        role_id = frontmatter.get("role_id")
        skills = frontmatter.get("skills")
        entries = frontmatter.get("allocations")

        if not isinstance(alias, str) or not alias.strip():
            errors.append(f"{path}: 'alias' must be a non-empty string")
            continue
        if path.stem != alias:
            errors.append(f"{path}: filename stem must match alias ('{alias}')")
        known_aliases.add(alias)

        if not isinstance(role_id, str) or not role_id.strip():
            errors.append(f"{path}: 'role_id' must be a non-empty string")
        elif known_roles and role_id not in known_roles:
            errors.append(f"{path}: unknown role_id '{role_id}' (not found in roles/)")

        if not isinstance(skills, list):
            errors.append(f"{path}: 'skills' must be a list")
        else:
            for idx, skill in enumerate(skills):
                if not isinstance(skill, str) or not skill.strip():
                    errors.append(f"{path}: skills[{idx}] must be a non-empty string")
                    continue
                normalized = normalize_skill(skill)
                canonical = skill_synonyms.get(normalized, normalized)
                if canonical_skills and canonical not in canonical_skills:
                    warnings.append(
                        f"{path}: unknown skill '{skill}' (allowed, but not in canonical_skills)"
                    )

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
            planned_hours = item.get("planned_hours")
            capacity_hours = item.get("capacity_hours")

            if not isinstance(project, str) or not project.strip():
                errors.append(f"{location}: 'project' must be a non-empty string")
            else:
                referenced_projects.add(project)

            if not isinstance(weeks, list) or not weeks:
                errors.append(f"{location}: 'weeks' must be a non-empty list")
                weeks = []

            if capacity_hours is None:
                capacity = DEFAULT_CAPACITY_HOURS
            elif isinstance(capacity_hours, (int, float)):
                capacity = float(capacity_hours)
            else:
                errors.append(f"{location}: 'capacity_hours' must be a number")
                continue
            if capacity <= 0:
                errors.append(f"{location}: 'capacity_hours' must be greater than 0")
                continue

            if planned_hours is not None:
                if not isinstance(planned_hours, (int, float)):
                    errors.append(f"{location}: 'planned_hours' must be a number")
                    continue
                hours = float(planned_hours)
                if hours < 0:
                    errors.append(f"{location}: 'planned_hours' must be >= 0")
                    continue

                if load is not None:
                    if not isinstance(load, int):
                        errors.append(f"{location}: 'load' must be an integer when provided")
                    else:
                        expected_load = round((hours / capacity) * 100) if capacity > 0 else 0
                        if abs(load - expected_load) > 1:
                            errors.append(
                                f"{location}: 'load' ({load}) is inconsistent with planned_hours/capacity_hours "
                                f"(expected about {expected_load})"
                            )
            else:
                if not isinstance(load, int):
                    errors.append(f"{location}: either 'planned_hours' or integer 'load' is required")
                    continue
                if load < 0:
                    errors.append(f"{location}: 'load' must be >= 0")
                    continue
                hours = (load / 100.0) * capacity

            for week in weeks:
                if not isinstance(week, str) or not ISO_WEEK_RE.match(week):
                    errors.append(f"{location}: invalid ISO week '{week}' (expected YYYY-Www)")
                    continue
                if week in capacities[alias] and abs(capacities[alias][week] - capacity) > 0.001:
                    errors.append(
                        f"{location}: conflicting capacity_hours for alias '{alias}' week {week} "
                        f"({capacities[alias][week]} vs {capacity})"
                    )
                    continue
                capacities[alias][week] = capacity
                totals[alias][week] += hours

    for alias, by_week in sorted(totals.items()):
        for week, total_hours in sorted(by_week.items()):
            capacity = capacities.get(alias, {}).get(week, DEFAULT_CAPACITY_HOURS)
            if total_hours > capacity + 1e-9:
                percent = round((total_hours / capacity) * 100, 1) if capacity > 0 else 0
                errors.append(
                    f"over-allocation: alias '{alias}' has total planned_hours {round(total_hours, 1)} "
                    f"in week {week} (capacity {round(capacity, 1)}h, {percent}%)"
                )

    return errors, warnings, totals, referenced_projects, known_aliases


def validate_allocations(people_dir: Path) -> tuple[list[str], dict[str, dict[str, float]], set[str]]:
    """Backward-compatible test helper name."""
    errors, _warnings, totals, ref_projects, _aliases = validate_people(
        people_dir=people_dir,
        known_roles=set(),
        canonical_skills=set(),
        skill_synonyms={},
    )
    return errors, totals, ref_projects


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
    parser.add_argument("--planning-dir", default="tst-data/planning", help="Path containing people/, roles/, projects/, and skills.md")
    parser.add_argument("--identity-dir", default="tst-data/identity", help="Path containing identity markdown files")
    args = parser.parse_args()

    planning_dir = Path(args.planning_dir)
    identity_dir = Path(args.identity_dir)
    people_dir = planning_dir / "people"
    roles_dir = planning_dir / "roles"
    projects_dir = planning_dir / "projects"
    skills_path = planning_dir / "skills.md"

    errors: list[str] = []
    if not people_dir.exists():
        errors.append(f"missing directory: {people_dir}")
    if not roles_dir.exists():
        errors.append(f"missing directory: {roles_dir}")
    if not projects_dir.exists():
        errors.append(f"missing directory: {projects_dir}")
    if not skills_path.exists():
        errors.append(f"missing file: {skills_path}")
    if errors:
        for err in errors:
            print(f"ERROR: {err}")
        return 1

    role_errors, role_warnings, role_names = validate_roles(roles_dir)
    skills_errors, skills_warnings, canonical_skills, skill_synonyms = validate_skills_catalog(skills_path)
    people_errors, people_warnings, totals, ref_projects, known_aliases = validate_people(
        people_dir=people_dir,
        known_roles=set(role_names.keys()),
        canonical_skills=canonical_skills,
        skill_synonyms=skill_synonyms,
    )
    project_errors, ref_aliases = validate_projects(projects_dir)

    known_projects = {p.stem for p in projects_dir.glob("*.md")}
    cross_errors: list[str] = []
    for proj in sorted(ref_projects):
        if proj not in known_projects:
            cross_errors.append(f"cross-reference: project '{proj}' referenced in people but not found in projects/")

    for alias in sorted(ref_aliases):
        if alias not in known_aliases:
            cross_errors.append(f"cross-reference: alias '{alias}' referenced in projects but not found in people/")

    real_names = load_real_names(identity_dir)
    pii_errors = check_pii_leaks(
        [*sorted(people_dir.glob("*.md")), *sorted(roles_dir.glob("*.md")), *sorted(projects_dir.glob("*.md")), skills_path],
        real_names,
    )

    all_warnings = [*role_warnings, *skills_warnings, *people_warnings]
    all_errors = [*role_errors, *skills_errors, *people_errors, *project_errors, *cross_errors, *pii_errors]

    for warning in all_warnings:
        print(f"WARNING: {warning}")
    if all_errors:
        for err in all_errors:
            print(f"ERROR: {err}")
        print(f"\nValidation failed with {len(all_errors)} error(s) and {len(all_warnings)} warning(s).")
        return 1

    print("Validation passed.")
    print(f"- people files: {len(known_aliases)}")
    print(f"- role files: {len(role_names)}")
    print(f"- project files: {len(known_projects)}")
    print(f"- weekly total slots computed: {sum(len(v) for v in totals.values())}")
    print(f"- warnings: {len(all_warnings)}")
    print(f"- identity files scanned: {len(list(identity_dir.glob('*.md')))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
