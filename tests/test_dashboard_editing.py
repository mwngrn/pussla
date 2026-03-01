import os
import sys
import tempfile
import unittest
from pathlib import Path

import yaml

# Add dashboard src to path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src', 'dashboard'))

import pussla_engine


class TestDashboardEditing(unittest.TestCase):
    @staticmethod
    def _read_frontmatter(path: Path) -> dict:
        text = path.read_text(encoding='utf-8')
        parts = text.split("\n---\n", 1)
        return yaml.safe_load(parts[0][4:]) or {}

    def test_update_week_allocations_replaces_week_only_and_supports_multi_project(self):
        with tempfile.TemporaryDirectory() as tmp:
            planning = Path(tmp) / 'planning'
            people_dir = planning / 'people'
            people_dir.mkdir(parents=True)
            path = people_dir / 'alice.md'
            path.write_text(
                """
---
alias: alice
role_id: Dev-Role
skills: [python]
allocations:
  - project: Project-A
    weeks: ["2026-W01", "2026-W02"]
    load: 50
  - project: Project-B
    weeks: ["2026-W02"]
    load: 20
---
Notes
""".lstrip(),
                encoding='utf-8',
            )

            result = pussla_engine.update_week_allocations(
                planning_dir=planning,
                alias='alice',
                week='2026-W02',
                allocations=[
                    {'project': 'Project-C', 'load': 70},
                    {'project': 'Project-D', 'load': 40},
                ],
            )

            self.assertEqual(result['alias'], 'alice')
            self.assertEqual(result['week'], '2026-W02')
            self.assertEqual(result['projects_count'], 2)
            self.assertEqual(result['total_load'], 110)

            data = self._read_frontmatter(path)
            by_project = {entry['project']: entry for entry in data['allocations']}

            # Existing unaffected week must remain.
            self.assertIn('2026-W01', by_project['Project-A']['weeks'])
            self.assertNotIn('2026-W02', by_project['Project-A']['weeks'])

            # Week should now exist only in newly saved projects.
            self.assertEqual(by_project['Project-C']['weeks'], ['2026-W02'])
            self.assertEqual(by_project['Project-D']['weeks'], ['2026-W02'])

    def test_update_week_allocations_validates_payload(self):
        with tempfile.TemporaryDirectory() as tmp:
            planning = Path(tmp) / 'planning'
            people_dir = planning / 'people'
            people_dir.mkdir(parents=True)
            (people_dir / 'alice.md').write_text(
                """
---
alias: alice
role_id: Dev-Role
skills: [python]
allocations: []
---
Notes
""".lstrip(),
                encoding='utf-8',
            )

            with self.assertRaises(ValueError):
                pussla_engine.update_week_allocations(
                    planning_dir=planning,
                    alias='alice',
                    week='bad',
                    allocations=[],
                )

            with self.assertRaises(ValueError):
                pussla_engine.update_week_allocations(
                    planning_dir=planning,
                    alias='alice',
                    week='2026-W03',
                    allocations=[{'project': 'X', 'load': -1}],
                )

    def test_update_week_allocations_supports_planned_hours(self):
        with tempfile.TemporaryDirectory() as tmp:
            planning = Path(tmp) / 'planning'
            people_dir = planning / 'people'
            people_dir.mkdir(parents=True)
            path = people_dir / 'alice.md'
            path.write_text(
                """
---
alias: alice
role_id: Dev-Role
skills: [python]
allocations: []
---
Notes
""".lstrip(),
                encoding='utf-8',
            )

            result = pussla_engine.update_week_allocations(
                planning_dir=planning,
                alias='alice',
                week='2026-W04',
                allocations=[
                    {'project': 'Project-Hours', 'planned_hours': 16, 'capacity_hours': 40},
                ],
            )

            self.assertEqual(result['total_planned_hours'], 16.0)
            self.assertEqual(result['total_load'], 40)

            data = self._read_frontmatter(path)
            entry = data['allocations'][0]
            self.assertEqual(entry['project'], 'Project-Hours')
            self.assertEqual(entry['planned_hours'], 16.0)
            self.assertEqual(entry['capacity_hours'], 40.0)
            self.assertEqual(entry['load'], 40)


if __name__ == '__main__':
    unittest.main()
