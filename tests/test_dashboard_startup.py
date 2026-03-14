import io
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "src", "dashboard"))

import run_dashboard


class TestDashboardStartup(unittest.TestCase):
    def test_resolve_data_dir_prefers_explicit_argument(self):
        with tempfile.TemporaryDirectory() as tmp:
            explicit = Path(tmp) / "custom-data"
            explicit.mkdir()

            data_dir, resolution_mode = run_dashboard._resolve_data_dir(
                str(explicit),
                cwd=Path(tmp) / "elsewhere",
            )

            self.assertEqual(data_dir, explicit.resolve())
            self.assertEqual(resolution_mode, "explicit --data-dir")

    def test_resolve_data_dir_auto_detects_current_directory(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "planning").mkdir()
            (root / "identity").mkdir()

            data_dir, resolution_mode = run_dashboard._resolve_data_dir(None, cwd=root)

            self.assertEqual(data_dir, root.resolve())
            self.assertEqual(resolution_mode, "auto-detected from current working directory")

    def test_resolve_data_dir_auto_detects_repo_tst_data(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            data_root = repo_root / "tst-data"
            (data_root / "planning").mkdir(parents=True)
            (data_root / "identity").mkdir()

            data_dir, _ = run_dashboard._resolve_data_dir(None, cwd=repo_root)

            self.assertEqual(data_dir, data_root.resolve())

    def test_validate_data_root_accepts_legacy_planing_folder(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            legacy = root / "planing"
            identity = root / "identity"
            legacy.mkdir()
            identity.mkdir()

            planning_dir, identity_dir = run_dashboard._validate_data_root(root)

            self.assertEqual(planning_dir, legacy)
            self.assertEqual(identity_dir, identity)

    def test_validate_data_root_reports_actionable_error(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            with self.assertRaises(SystemExit) as ctx:
                run_dashboard._validate_data_root(root)

            message = str(ctx.exception)
            self.assertIn("Invalid dashboard data root", message)
            self.assertIn("planning", message)
            self.assertIn("--data-dir", message)

    def test_launch_browser_uses_resolved_url(self):
        calls = []

        def opener(url: str) -> bool:
            calls.append(url)
            return True

        launched = run_dashboard._launch_browser(
            run_dashboard._dashboard_url("127.0.0.1", 8081),
            opener=opener,
            stream=io.StringIO(),
        )

        self.assertTrue(launched)
        self.assertEqual(calls, ["http://127.0.0.1:8081"])

    def test_launch_browser_warns_without_failing_startup(self):
        output = io.StringIO()

        def opener(_: str) -> bool:
            return False

        launched = run_dashboard._launch_browser(
            "http://127.0.0.1:8081",
            opener=opener,
            stream=output,
        )

        self.assertFalse(launched)
        self.assertIn("Open http://127.0.0.1:8081 manually", output.getvalue())


if __name__ == "__main__":
    unittest.main()
