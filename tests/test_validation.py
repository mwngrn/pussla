import unittest
import sys
import os
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

if __name__ == '__main__':
    unittest.main()
