from __future__ import annotations

import json
import os
import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from slotbench.stats import Arrival, board, percentile_nearest_rank, relative_delays  # noqa: E402

BIN = [sys.executable, "-m", "slotbench"]


def run(args):
    return subprocess.run(
        BIN + args,
        cwd=ROOT,
        capture_output=True,
        text=True,
        env={**os.environ, "PYTHONPATH": str(ROOT)},
    )


class TestStats(unittest.TestCase):
    def test_percentile_known_list(self):
        s = [0, 10, 20, 30, 40]
        self.assertEqual(percentile_nearest_rank(s, 50), 20)
        self.assertEqual(percentile_nearest_rank(s, 100), 40)

    def test_relative_and_two_endpoints(self):
        arr = [
            Arrival(1, "a", 1000),
            Arrival(1, "b", 1300),
            Arrival(2, "a", 2000),
            Arrival(2, "b", 2000),
        ]
        d = relative_delays(arr)
        self.assertEqual(d["a"], [0, 0])
        self.assertEqual(d["b"], [300, 0])
        with self.assertRaises(ValueError):
            relative_delays([Arrival(1, "a", 1)])

    def test_board_fixture(self):
        rows = json.loads((ROOT / "fixtures/arrivals.json").read_text())["arrivals"]
        from slotbench.stats import arrivals_from_rows

        b = board(arrivals_from_rows(rows))
        self.assertIn("alpha", b["endpoints"])
        self.assertIn("beta", b["endpoints"])
        self.assertEqual(b["endpoints"]["alpha"]["p50"], 0)


class TestCLI(unittest.TestCase):
    def test_bench_twice_same(self):
        r1 = run(["bench", "--fixture", str(ROOT / "fixtures/arrivals.json")])
        r2 = run(["bench", "--fixture", str(ROOT / "fixtures/arrivals.json")])
        self.assertEqual(r1.returncode, 0, r1.stderr)
        self.assertEqual(r2.returncode, 0, r2.stderr)
        self.assertEqual(json.loads(r1.stdout), json.loads(r2.stdout))
        self.assertIn("p50", r1.stdout)
        self.assertIn("p99", r1.stdout)

    def test_one_endpoint_fails(self):
        r = run(["bench", "--fixture", str(ROOT / "fixtures/one_endpoint.json")])
        self.assertNotEqual(r.returncode, 0)

    def test_doctor_secret(self):
        r = run(["doctor", "--config", str(ROOT / "fixtures/config.secret.json")])
        self.assertNotEqual(r.returncode, 0)
        self.assertNotIn("PLANT-SECRET-DO-NOT-LOG", r.stdout + r.stderr)


if __name__ == "__main__":
    unittest.main()
