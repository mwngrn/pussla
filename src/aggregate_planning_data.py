#!/usr/bin/env python3
"""Aggregate Pussla planning data for monthly/quarterly summaries."""

from __future__ import annotations

import argparse
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any
from datetime import datetime

import yaml


def read_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def parse_frontmatter(path: Path) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return {}
    parts = text.split("\n---\n", 1)
    if len(parts) != 2:
        return {}
    data = yaml.safe_load(parts[0][4:]) or {}
    return data if isinstance(data, dict) else {}


def get_month_from_iso_week(iso_week: str) -> str:
    """Map YYYY-Www to YYYY-MM based on the first day of the ISO week."""
    # datetime.strptime(..., "%G-W%V-%u") is supported in 3.6+
    d = datetime.strptime(iso_week + '-1', "%G-W%V-%u")
    return d.strftime("%Y-%m")


def aggregate_data(people_dir: Path) -> dict[str, dict[str, int]]:
    """Sum up total load per alias per week."""
    totals: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    for path in sorted(people_dir.glob("*.md")):
        data = parse_frontmatter(path)
        if not data or not isinstance(data, dict):
            continue
        alias = data.get("alias")
        entries = data.get("allocations", [])
        for entry in entries:
            load = entry.get("load", 0)
            weeks = entry.get("weeks", [])
            for week in weeks:
                totals[alias][week] += load
    return totals


def generate_summary(totals: dict[str, dict[str, int]]):
    """Generate a summary report."""
    all_weeks = sorted({w for by_week in totals.values() for w in by_week})
    if not all_weeks:
        print("No planning data found.")
        return

    print("# Pussla Weekly Allocation Summary\n")
    print("| Alias | " + " | ".join(all_weeks) + " |")
    print("| :--- | " + " | ".join(["---:"] * len(all_weeks)) + " |")

    for alias in sorted(totals.keys()):
        row = [alias]
        for week in all_weeks:
            load = totals[alias].get(week, 0)
            status = " "
            if load > 100: status = "🚨"
            elif load == 100: status = "✅"
            elif load > 0: status = f"{load}%"
            else: status = "-"
            row.append(status)
        print("| " + " | ".join(row) + " |")


def main() -> int:
    parser = argparse.ArgumentParser(description="Aggregate Pussla planning data")
    parser.add_argument("--planning-dir", default="tst-data/planning", help="Path containing people/")
    args = parser.parse_args()

    planning_dir = Path(args.planning_dir)
    people_dir = planning_dir / "people"

    if not people_dir.exists():
        print(f"ERROR: missing directory: {people_dir}")
        return 1

    totals = aggregate_data(people_dir)
    generate_summary(totals)
    return 0


if __name__ == "__main__":
    sys.exit(main())
