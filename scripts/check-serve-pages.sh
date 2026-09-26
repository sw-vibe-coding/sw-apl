#!/usr/bin/env bash
# serve-pages.py on a port something else holds: it must say so in a
# line, name the port and what to do, and exit non-zero -- not print a
# Python traceback. Run by `just gates`.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
python3 - <<'PY'
import socket, subprocess, sys
held = socket.socket()
held.bind(("127.0.0.1", 0))
held.listen()
port = held.getsockname()[1]
run = subprocess.run([sys.executable, "scripts/serve-pages.py", str(port), "pages"],
                     capture_output=True, text=True, timeout=20)
said = run.stderr + run.stdout
bad = []
if run.returncode == 0: bad.append("exited 0")
if "Traceback" in said: bad.append("printed a traceback")
if f"{port} is in use" not in said: bad.append("did not name the port")
if "just pages-serve" not in said: bad.append("did not say how to pick another")
if bad:
    print("check-serve-pages: " + "; ".join(bad) + f"\n{said}")
    sys.exit(1)
print("check-serve-pages: a taken port is said plainly")
PY
