#!/usr/bin/env python3
# The reg-rs transcript regressions, run without reg-rs: each
# tests/reg-rs/*.rgt's command through bash, its preprocess filter over
# the output and over the .out baseline, then the comparison and the
# exit code, as reg-rs does. For a machine reg-rs is not installed on --
# `just reg-linux` runs it on Linux, in Docker. Anything a command
# writes to stderr is reported too: a shell warning is a portability
# bug even when the transcript matches.
#
# Usage: scripts/reg-portable.py     (from the repository root, after
#                                     a release build of sw-apl)
import glob, subprocess, sys, tomllib
failed = []
for rgt in sorted(glob.glob("tests/reg-rs/*.rgt")):
    spec = tomllib.load(open(rgt, "rb"))
    base = rgt[:-4] + ".out"
    run = subprocess.run(["bash", "-c", spec["command"]], capture_output=True, timeout=spec.get("timeout", 60))
    pre = spec.get("preprocess")
    norm = lambda b: subprocess.run(["bash", "-c", pre], input=b, capture_output=True).stdout if pre else b
    got, want = norm(run.stdout), norm(open(base, "rb").read())
    ok = got == want and run.returncode == spec.get("exit_code", 0)
    if not ok or run.stderr:
        failed.append((rgt, ok, run.stderr.decode()[:200]))
for rgt, ok, err in failed:
    print(("FAIL " if not ok else "WARN ") + rgt, err.strip())
ran = len(glob.glob("tests/reg-rs/*.rgt"))
print(f"{ran} run, {sum(1 for f in failed if not f[1])} failed, {sum(1 for f in failed if f[1])} with stderr")
sys.exit(1 if failed else 0)
