import os
import sys
import tempfile
import unittest
from pathlib import Path

import yaml

sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src', 'dashboard'))

import pussla_engine


class TestProjectManagementBackend(unittest.TestCase):
    def test_update_project_metadata_milestones_and_rate(self):
        with tempfile.TemporaryDirectory() as tmp:
            planning = Path(tmp) / 'planning'
            projects = planning / 'projects'
            projects.mkdir(parents=True)
            path = projects / 'Project-X.md'
            path.write_text(
                """
---
project_id: project-x
name: Project-X
owner_alias: alice
start_week: "2026-W10"
end_week: "2026-W14"
status: active
team_aliases: [alice]
---
Body
""".lstrip(),
                encoding='utf-8',
            )

            result = pussla_engine.update_project_metadata(
                planning_dir=planning,
                project='Project-X',
                updates={
                    'hourly_rate': 130,
                    'start_week_override': '2026-W09',
                    'end_week_override': '2026-W15',
                    'milestones': [
                        {'title': 'Go Live', 'date': '2026-03-01'},
                        {'title': 'Kickoff', 'date': '2026-02-01'},
                    ],
                },
            )

            self.assertEqual(result['project'], 'Project-X')
            content = path.read_text(encoding='utf-8')
            self.assertIn('hourly_rate: 130.0', content)
            self.assertIn('start_week_override: 2026-W09', content)
            self.assertIn('end_week_override: 2026-W15', content)
            kickoff_idx = content.index('Kickoff')
            golive_idx = content.index('Go Live')
            self.assertLess(kickoff_idx, golive_idx)

    def test_build_dashboard_data_exposes_projects_and_assignment_state(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            planning = root / 'planning'
            allocations = planning / 'allocations'
            projects_dir = planning / 'projects'
            identity = root / 'identity'
            allocations.mkdir(parents=True)
            projects_dir.mkdir(parents=True)
            identity.mkdir(parents=True)

            (projects_dir / 'Project-X.md').write_text(
                """
---
project_id: project-x
name: Project-X
owner_alias: alice
start_week: "2026-W10"
end_week: "2026-W12"
status: active
hourly_rate: 120
milestones:
  - id: ms-1
    title: Kickoff
    date: 2026-02-01
team_aliases: [alice]
---
Body
""".lstrip(),
                encoding='utf-8',
            )

            (allocations / 'alice.yaml').write_text(
                yaml.safe_dump(
                    {
                        'alias': 'alice',
                        'allocations': [
                            {
                                'project': 'Project-X',
                                'weeks': ['2026-W10'],
                                'planned_hours': 16,
                                'capacity_hours': 40,
                                'load': 40,
                                'state': 'tentative',
                            }
                        ],
                    },
                    sort_keys=False,
                    allow_unicode=True,
                ),
                encoding='utf-8',
            )

            data = pussla_engine.build_dashboard_data(
                planning_dir=planning,
                identity_dir=identity,
                include_pii=False,
            )

            self.assertIn('projects', data)
            self.assertTrue(any(p['name'] == 'Project-X' for p in data['projects']))

            user = data['users'][0]
            slot = next(s for s in user['weekly_stats'] if s['week'] == '2026-W10')
            self.assertAlmostEqual(slot['total_planned_hours'], 16.0)
            self.assertAlmostEqual(slot['capacity_hours'], 40.0)
            self.assertEqual(slot['projects'][0]['state'], 'tentative')

    def test_build_dashboard_data_reads_milestone_dates_from_yaml_date_type(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            planning = root / 'planning'
            allocations = planning / 'allocations'
            projects_dir = planning / 'projects'
            identity = root / 'identity'
            allocations.mkdir(parents=True)
            projects_dir.mkdir(parents=True)
            identity.mkdir(parents=True)

            (projects_dir / 'Project-Y.md').write_text(
                """
---
project_id: project-y
name: Project-Y
owner_alias: alice
start_week: \"2026-W10\"
end_week: \"2026-W12\"
status: active
milestones:
  - id: ms-1
    title: Kickoff
    date: 2026-02-01
team_aliases: [alice]
---
Body
""".lstrip(),
                encoding='utf-8',
            )

            (allocations / 'alice.yaml').write_text(
                """
alias: alice
allocations: []
""".lstrip(),
                encoding='utf-8',
            )

            data = pussla_engine.build_dashboard_data(
                planning_dir=planning,
                identity_dir=identity,
                include_pii=False,
            )

            project = next(p for p in data['projects'] if p['name'] == 'Project-Y')
            self.assertEqual(project['milestones'][0]['date'], '2026-02-01')


if __name__ == '__main__':
    unittest.main()
