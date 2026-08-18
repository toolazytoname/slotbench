from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from slotbench.secrets import forbidden_fields
from slotbench.stats import arrivals_from_rows, board


def cmd_doctor(args) -> int:
    cfg = json.loads(Path(args.config).read_text(encoding="utf-8"))
    hits = forbidden_fields(cfg)
    if hits:
        print(f"doctor: forbidden secret field(s): {', '.join(hits)}", file=sys.stderr)
        return 2
    print("ok method=docs/METHOD.md")
    return 0


def cmd_bench(args) -> int:
    payload = json.loads(Path(args.fixture).read_text(encoding="utf-8"))
    try:
        result = board(arrivals_from_rows(payload["arrivals"]))
    except ValueError as e:
        print(f"bench: {e}", file=sys.stderr)
        return 2
    text = json.dumps(result, indent=2, sort_keys=True) + "\n"
    print(text, end="")
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
    return 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="slotbench")
    sub = p.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("doctor")
    s.add_argument("--config", required=True)
    s.set_defaults(func=cmd_doctor)
    s = sub.add_parser("bench")
    s.add_argument("--fixture", required=True)
    s.add_argument("--out", default=None)
    s.set_defaults(func=cmd_bench)
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)
