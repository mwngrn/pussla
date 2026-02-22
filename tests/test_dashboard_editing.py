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
    def test_update_week_allocations_replaces_week_only_and_supports_multi_project(self):
        with tempfile.TemporaryDirectory() as tmp:
            planning = Path(tmp) / 'planning'
            alloc_dir = planning / 'allocations'
            alloc_dir.mkdir(parents=True)
            path = alloc_dir / 'alice.yaml'
            path.write_text(
                """
alias: alice
allocations:
  - project: Project-A
    weeks: ["2026-W01", "2026-W02"]
    load: 50
  - project: Project-B
    weeks: ["2026-W02"]
    load: 20
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

            data = yaml.safe_load(path.read_text(encoding='utf-8'))
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
            alloc_dir = planning / 'allocations'
            alloc_dir.mkdir(parents=True)
            (alloc_dir / 'alice.yaml').write_text(
                """
alias: alice
allocations: []
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


if __name__ == '__main__':
    unittest.main()
