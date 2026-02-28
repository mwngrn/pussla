import unittest
import sys
import os
import tempfile
from pathlib import Path

# Add src to path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

import validate_planning_data

class TestValidation(unittest.TestCase):
    def test_iso_week_regex(self):
        self.assertTrue(validate_planning_data.ISO_WEEK_RE.match("2026-W01"))
        self.assertTrue(validate_planning_data.ISO_WEEK_RE.match("2026-W53"))
        self.assertFalse(validate_planning_data.ISO_WEEK_RE.match("2026-W00"))
        self.assertFalse(validate_planning_data.ISO_WEEK_RE.match("2026-W54"))
        self.assertFalse(validate_planning_data.ISO_WEEK_RE.match("26-W01"))

    def test_email_pii_regex(self):
        self.assertTrue(validate_planning_data.EMAIL_RE.search("test@example.com"))
        self.assertTrue(validate_planning_data.EMAIL_RE.search("My email is erik.a@company.se."))
        self.assertFalse(validate_planning_data.EMAIL_RE.search("not an email"))

    def test_validate_allocations_supports_planned_hours(self):
        with tempfile.TemporaryDirectory() as tmp:
            allocations_dir = Path(tmp)
            (allocations_dir / "alice.yaml").write_text(
                """
alias: alice
allocations:
  - project: Project-A
    weeks: ["2026-W10"]
    planned_hours: 16
    capacity_hours: 40
""".lstrip(),
                encoding="utf-8",
            )

            errors, totals, ref_projects = validate_planning_data.validate_allocations(
                allocations_dir
            )

            self.assertEqual(errors, [])
            self.assertAlmostEqual(totals["alice"]["2026-W10"], 16.0)
            self.assertIn("Project-A", ref_projects)

    def test_validate_allocations_flags_overallocation_by_hours(self):
        with tempfile.TemporaryDirectory() as tmp:
            allocations_dir = Path(tmp)
            (allocations_dir / "alice.yaml").write_text(
                """
alias: alice
allocations:
  - project: Project-A
    weeks: ["2026-W10"]
    planned_hours: 30
    capacity_hours: 40
  - project: Project-B
    weeks: ["2026-W10"]
    planned_hours: 20
    capacity_hours: 40
""".lstrip(),
                encoding="utf-8",
            )

            errors, _totals, _ref_projects = validate_planning_data.validate_allocations(
                allocations_dir
            )

            self.assertTrue(
                any("over-allocation" in err and "planned_hours" in err for err in errors)
            )

if __name__ == '__main__':
    unittest.main()
