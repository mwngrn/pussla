import unittest
import sys
import os
from pathlib import Path

# Add src to path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

import aggregate_planning_data

class TestAggregation(unittest.TestCase):
    def test_get_month_from_iso_week(self):
        # 2026-W09 starts on 2026-02-23
        self.assertEqual(aggregate_planning_data.get_month_from_iso_week("2026-W09"), "2026-02")
        # 2026-W10 starts on 2026-03-02
        self.assertEqual(aggregate_planning_data.get_month_from_iso_week("2026-W10"), "2026-03")

if __name__ == '__main__':
    unittest.main()
