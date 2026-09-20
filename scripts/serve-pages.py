#!/usr/bin/env python3
"""Serve pages/ the way a static host would, but isolated.

`SharedArrayBuffer` needs a cross-origin isolated page, which means
two headers. GitHub Pages will not send them, which is why the demo
carries a service worker that adds them to its own responses and
loads a second time under it. Locally there is no reason to make a
reader pay for that: this sends the headers, so the page is isolated
on the first response, loads once, and never registers the worker.

The service-worker path is still what a published demo uses, and
`just check-pages` deliberately serves without these headers so that
it stays covered.
"""

import functools
import http.server
import sys

ISOLATION = {
    "Cross-Origin-Opener-Policy": "same-origin",
    "Cross-Origin-Embedder-Policy": "require-corp",
    "Cross-Origin-Resource-Policy": "cross-origin",
}


class Isolated(http.server.SimpleHTTPRequestHandler):
    """A static host that says the page may have shared memory."""

    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
        ".json": "application/json",
    }

    def end_headers(self):
        for name, value in ISOLATION.items():
            self.send_header(name, value)
        # The demo is rebuilt under the reader's own feet often enough
        # that a cached copy is a bug report waiting to happen; the
        # version stamp is for published visitors, not for this.
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def log_message(self, fmt, *args):
        pass


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8361
    root = sys.argv[2] if len(sys.argv) > 2 else "pages"
    handler = functools.partial(Isolated, directory=root)
    with http.server.ThreadingHTTPServer(("127.0.0.1", port), handler) as httpd:
        print(f"sw-apl in a browser: http://127.0.0.1:{port}/")
        httpd.serve_forever()


if __name__ == "__main__":
    main()
