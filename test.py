#!/usr/bin/env python3
"""Test harness: runs QEMU and monitors serial output for success/failure.

Spawns the QEMU command passed as arguments, watches the serial console for
`TEST_PASS` or `TEST_FAIL`, and exits accordingly.

# Usage

```bash
python3 test.py qemu-system-x86_64 -machine q35 -m 2G ...
```

# Exit codes

- `0` — `TEST_PASS` seen in serial output before timeout
- `1` — `TEST_FAIL` seen or `TEST_PASS` not seen within timeout
- `2` — Python/pexpect error (missing dependency, spawn failure, etc.)
"""

import sys
import argparse

try:
    import pexpect
except ImportError:
    pexpect = None  # type: ignore[assignment]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run QEMU and monitor serial output for test results."
    )
    parser.add_argument(
        "cmd",
        nargs=argparse.REMAINDER,
        help="QEMU command and arguments to spawn",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=30,
        help="Seconds to wait for TEST_PASS (default: 30)",
    )
    args = parser.parse_args()

    if not args.cmd:
        parser.error("No QEMU command provided")

    if pexpect is None:
        print(
            "error: pexpect is required. Install with: pip install pexpect",
            file=sys.stderr,
        )
        return 2

    return run(args.cmd, args.timeout)


def run(cmd: list[str], timeout: int) -> int:
    """Spawn QEMU and wait for TEST_PASS in serial output.

    Returns 0 on success, 1 on failure/timeout.
    """
    child = pexpect.spawn(cmd[0], cmd[1:], encoding="utf-8", timeout=timeout)
    child.logfile = sys.stdout

    # Wait for the success marker printed by the root task.
    # If we see "TEST_FAIL" first, the test has already failed.
    index = child.expect(["TEST_PASS", "TEST_FAIL", pexpect.EOF])

    if index == 0:
        print("\n[test.py] TEST_PASS detected — success", file=sys.stderr)
        child.terminate(force=True)
        return 0
    if index == 1:
        print("\n[test.py] TEST_FAIL detected — failure", file=sys.stderr)
        child.terminate(force=True)
        return 1

    # EOF before any marker
    print(
        f"\n[test.py] QEMU exited before TEST_PASS (timeout={timeout}s)",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
