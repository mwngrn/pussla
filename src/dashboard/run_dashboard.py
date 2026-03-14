#!/usr/bin/env python3
from __future__ import annotations

import argparse
import errno
import json
import sys
import webbrowser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

from dashboard_sync import DashboardSyncError, get_sync_status, publish_changes, refresh_changes
from pussla_engine import (
    build_dashboard_data,
    update_project_metadata,
    update_week_allocations,
)


class DashboardHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, static_dir: Path, **kwargs):
        self._static_dir = static_dir
        super().__init__(*args, directory=str(static_dir), **kwargs)

    def do_GET(self) -> None:  # noqa: N802
        parsed = urlparse(self.path)

        if parsed.path == "/api/dashboard-data":
            query = parse_qs(parsed.query)
            include_pii = query.get("include_pii", ["1"])[0] != "0"
            data = build_dashboard_data(
                planning_dir=self.server.planning_dir,
                identity_dir=self.server.identity_dir,
                include_pii=include_pii,
            )
            payload = json.dumps(data, ensure_ascii=False).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
            return

        if parsed.path == "/api/sync/status":
            try:
                payload = json.dumps(
                    get_sync_status(self.server.data_dir), ensure_ascii=False
                ).encode("utf-8")
            except DashboardSyncError as exc:
                self._send_json(
                    exc.status_code,
                    {"error": str(exc), **(exc.details or {})},
                )
                return
            self.send_response(200)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
            return

        # For the React SPA: any non-asset path that doesn't match a file
        # falls back to index.html so client-side routing works.
        if parsed.path == "/" or (
            not parsed.path.startswith("/assets/")
            and not parsed.path.startswith("/api/")
            and "." not in Path(parsed.path).name
        ):
            self.path = "/index.html"

        super().do_GET()

    def _send_json(self, status: int, payload: dict) -> None:
        body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self) -> None:  # noqa: N802
        parsed = urlparse(self.path)
        normalized_path = parsed.path.rstrip("/") or "/"
        if normalized_path not in {
            "/api/allocation/update",
            "/api/project/update",
            "/api/projects/update",
            "/api/sync/publish",
            "/api/sync/refresh",
        }:
            self._send_json(404, {"error": "Not found"})
            return

        try:
            content_length = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            self._send_json(400, {"error": "Invalid Content-Length header"})
            return

        if content_length <= 0:
            self._send_json(400, {"error": "Request body is required"})
            return

        raw_body = self.rfile.read(content_length)
        try:
            payload = json.loads(raw_body.decode("utf-8"))
        except Exception:
            self._send_json(400, {"error": "Invalid JSON body"})
            return

        if not isinstance(payload, dict):
            self._send_json(400, {"error": "JSON body must be an object"})
            return

        if normalized_path == "/api/sync/publish":
            summary = payload.get("summary")
            if not isinstance(summary, str):
                self._send_json(400, {"error": "summary must be a string"})
                return
            try:
                result = publish_changes(self.server.data_dir, summary)
            except DashboardSyncError as exc:
                self._send_json(
                    exc.status_code,
                    {"error": str(exc), **(exc.details or {})},
                )
                return
            self._send_json(200, {"ok": True, **result})
            return

        if normalized_path == "/api/sync/refresh":
            try:
                result = refresh_changes(self.server.data_dir)
            except DashboardSyncError as exc:
                self._send_json(
                    exc.status_code,
                    {"error": str(exc), **(exc.details or {})},
                )
                return
            self._send_json(200, {"ok": True, **result})
            return

        if normalized_path == "/api/allocation/update":
            alias = payload.get("alias")
            week = payload.get("week")
            allocations = payload.get("allocations")
            try:
                result = update_week_allocations(
                    planning_dir=self.server.planning_dir,
                    alias=alias,
                    week=week,
                    allocations=allocations,
                )
            except FileNotFoundError as exc:
                self._send_json(404, {"error": str(exc)})
                return
            except ValueError as exc:
                self._send_json(400, {"error": str(exc)})
                return
            except Exception:
                self._send_json(500, {"error": "Failed to update allocation"})
                return
            self._send_json(200, {"ok": True, "updated": result})
            return

        project = payload.get("project")
        updates = payload.get("updates")
        try:
            result = update_project_metadata(
                planning_dir=self.server.planning_dir,
                project=project,
                updates=updates,
            )
        except FileNotFoundError as exc:
            self._send_json(404, {"error": str(exc)})
            return
        except ValueError as exc:
            self._send_json(400, {"error": str(exc)})
            return
        except Exception:
            self._send_json(500, {"error": "Failed to update project metadata"})
            return

        self._send_json(200, {"ok": True, "updated": result})


class DashboardServer(ThreadingHTTPServer):
    allow_reuse_address = True
    data_dir: Path
    planning_dir: Path
    identity_dir: Path


def _resolve_planning_dir(data_dir: Path) -> Path:
    preferred = data_dir / "planning"
    legacy = data_dir / "planing"
    if preferred.exists() or not legacy.exists():
        return preferred
    return legacy


def _is_valid_data_root(candidate: Path) -> bool:
    planning_dir = _resolve_planning_dir(candidate)
    identity_dir = candidate / "identity"
    return planning_dir.is_dir() and identity_dir.is_dir()


def _format_data_root_error(candidates: list[Path]) -> str:
    tried = "\n".join(f"- {candidate}" for candidate in candidates)
    return (
        "Cannot resolve a valid dashboard data root.\n"
        "Expected a folder containing planning/ (or legacy planing/) and identity/.\n"
        f"Tried:\n{tried}\n"
        "Examples:\n"
        "  Auto-detect: cd /path/to/data-root && python3 src/dashboard/run_dashboard.py\n"
        "  Explicit:    python3 src/dashboard/run_dashboard.py --data-dir /path/to/data-root"
    )


def _resolve_data_dir(
    explicit_data_dir: str | None,
    cwd: Path | None = None,
) -> tuple[Path, str]:
    if explicit_data_dir:
        data_dir = Path(explicit_data_dir).expanduser().resolve()
        return data_dir, "explicit --data-dir"

    current_dir = (cwd or Path.cwd()).resolve()
    candidates = [current_dir, current_dir / "tst-data"]
    for candidate in candidates:
        if _is_valid_data_root(candidate):
            return candidate, "auto-detected from current working directory"

    raise SystemExit(_format_data_root_error(candidates))


def _validate_data_root(data_dir: Path) -> tuple[Path, Path]:
    planning_dir = _resolve_planning_dir(data_dir)
    identity_dir = data_dir / "identity"
    problems: list[str] = []

    if not planning_dir.is_dir():
        problems.append(
            f"- Missing planning directory: expected {data_dir / 'planning'} "
            f"(legacy {data_dir / 'planing'} is also accepted)"
        )
    if not identity_dir.is_dir():
        problems.append(f"- Missing identity directory: expected {identity_dir}")

    if problems:
        message = (
            f"Invalid dashboard data root: {data_dir}\n"
            + "\n".join(problems)
            + "\nExamples:\n"
            + "  Auto-detect: cd /path/to/data-root && python3 src/dashboard/run_dashboard.py\n"
            + "  Explicit:    python3 src/dashboard/run_dashboard.py --data-dir /path/to/data-root"
        )
        raise SystemExit(message)

    return planning_dir, identity_dir


def _resolve_static_dir(static_override: str | None = None) -> Path:
    """Return the directory to serve static files from.

    Prefers the compiled React frontend (src/frontend/dist/) when present,
    and falls back to the legacy HTML dashboard in the same directory.
    """
    if static_override:
        override_path = Path(static_override)
        if not (override_path / "index.html").exists():
            raise SystemExit(
                f"Static dir does not contain index.html: {override_path}"
            )
        return override_path

    here = Path(__file__).resolve().parent
    react_dist = here.parent / "frontend" / "dist"
    if (react_dist / "index.html").exists():
        return react_dist
    return here


def _dashboard_url(host: str, resolved_port: int) -> str:
    return f"http://{host}:{resolved_port}"


def _launch_browser(url: str, *, opener=webbrowser.open, stream=None) -> bool:
    output = stream or sys.stderr
    try:
        if opener(url):
            return True
    except Exception as exc:
        print(
            f"Warning: could not open the default browser automatically ({exc}). "
            f"Open {url} manually.",
            file=output,
        )
        return False

    print(
        f"Warning: could not open the default browser automatically. Open {url} manually.",
        file=output,
    )
    return False


def run_server(
    host: str,
    port: int,
    data_dir: Path,
    planning_dir: Path,
    identity_dir: Path,
    static_dir_override: str | None = None,
    launch_browser: bool = False,
) -> None:
    static_dir = _resolve_static_dir(static_dir_override)

    def handler(*args, **kwargs):
        return DashboardHandler(*args, static_dir=static_dir, **kwargs)

    try:
        server = DashboardServer((host, port), handler)
    except OSError as exc:
        if exc.errno == errno.EADDRINUSE:
            raise SystemExit(
                f"Cannot start dashboard: {host}:{port} is already in use. "
                "Stop the existing process or choose another port with --port "
                "(tip: use --port 0 to auto-select a free port)."
            ) from exc
        raise

    server.data_dir = data_dir
    server.planning_dir = planning_dir
    server.identity_dir = identity_dir

    resolved_port = server.server_address[1]
    url = _dashboard_url(host, resolved_port)
    print(f"Pussla dashboard running at {url}")
    print(f"Frontend:      {static_dir}")
    print(f"Planning data: {planning_dir}")
    print(f"Identity data: {identity_dir}")
    print("Press Ctrl+C to stop.")

    if launch_browser:
        _launch_browser(url)

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


def main() -> None:
    parser = argparse.ArgumentParser(description="Run local Pussla dashboard")
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument(
        "--data-dir",
        default=None,
        help="Explicit data root containing planning/ (or legacy planing/) and identity/. "
        "If omitted, startup auto-detects from the current directory or ./tst-data.",
    )
    parser.add_argument("--static-dir", default=None, help="Override static frontend directory (must contain index.html)")
    parser.add_argument(
        "--launch-browser",
        action="store_true",
        help="Open the dashboard URL in the default browser after startup succeeds.",
    )
    args = parser.parse_args()

    data_dir, resolution_mode = _resolve_data_dir(args.data_dir)
    planning_dir, identity_dir = _validate_data_root(data_dir)
    print(f"Data root:     {data_dir} ({resolution_mode})")

    run_server(
        host=args.host,
        port=args.port,
        data_dir=data_dir,
        planning_dir=planning_dir,
        identity_dir=identity_dir,
        static_dir_override=args.static_dir,
        launch_browser=args.launch_browser,
    )


if __name__ == "__main__":
    main()
