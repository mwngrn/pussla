#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

from pussla_engine import build_dashboard_data


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


class DashboardServer(ThreadingHTTPServer):
    planning_dir: Path
    identity_dir: Path


def run_server(host: str, port: int, planning_dir: Path, identity_dir: Path) -> None:
    static_dir = Path(__file__).resolve().parent

    def handler(*args, **kwargs):
        return DashboardHandler(*args, static_dir=static_dir, **kwargs)

    server = DashboardServer((host, port), handler)
    server.planning_dir = planning_dir
    server.identity_dir = identity_dir

    url = f"http://{host}:{port}"
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
    parser.add_argument("--data-dir", default="tst-data", help="Base folder containing planing/ and identity/")
    parser.add_argument("--planning-dir", default=None, help="Override planning folder (contains allocations/ and projects/)")
    parser.add_argument("--identity-dir", default=None, help="Override identity folder")
    args = parser.parse_args()

    data_dir = Path(args.data_dir)
    planning_dir = Path(args.planning_dir) if args.planning_dir else data_dir / "planing"
    identity_dir = Path(args.identity_dir) if args.identity_dir else data_dir / "identity"

    run_server(
        host=args.host,
        port=args.port,
        planning_dir=planning_dir,
        identity_dir=identity_dir,
    )


if __name__ == "__main__":
    main()
