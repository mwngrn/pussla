#!/usr/bin/env python3
from __future__ import annotations

import argparse
import errno
import json
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

from pussla_engine import build_dashboard_data, update_week_allocations


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

        if parsed.path == "/":
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
        if parsed.path != "/api/allocation/update":
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


class DashboardServer(ThreadingHTTPServer):
    allow_reuse_address = True
    planning_dir: Path
    identity_dir: Path


def _resolve_planning_dir(data_dir: Path, planning_override: str | None) -> Path:
    if planning_override:
        return Path(planning_override)

    preferred = data_dir / "planning"
    legacy = data_dir / "planing"
    if preferred.exists() or not legacy.exists():
        return preferred
    return legacy


def run_server(host: str, port: int, planning_dir: Path, identity_dir: Path) -> None:
    static_dir = Path(__file__).resolve().parent

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

    server.planning_dir = planning_dir
    server.identity_dir = identity_dir

    resolved_port = server.server_address[1]
    url = f"http://{host}:{resolved_port}"
    print(f"Pussla dashboard running at {url}")
    print(f"Planning data: {planning_dir}")
    print(f"Identity data: {identity_dir}")
    print("Press Ctrl+C to stop.")

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
    parser.add_argument("--data-dir", default="tst-data", help="Base folder containing planning/ (or legacy planing/) and identity/")
    parser.add_argument("--planning-dir", default=None, help="Override planning folder (contains allocations/ and projects/)")
    parser.add_argument("--identity-dir", default=None, help="Override identity folder")
    args = parser.parse_args()

    data_dir = Path(args.data_dir)
    planning_dir = _resolve_planning_dir(data_dir, args.planning_dir)
    identity_dir = Path(args.identity_dir) if args.identity_dir else data_dir / "identity"

    run_server(
        host=args.host,
        port=args.port,
        planning_dir=planning_dir,
        identity_dir=identity_dir,
    )


if __name__ == "__main__":
    main()
