"""Relative arrival percentiles. All times are integer microseconds."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Arrival:
    slot: int
    endpoint: str
    recv_us: int


def relative_delays(arrivals: list[Arrival]) -> dict[str, list[int]]:
    """For each slot, earliest recv_us is 0; others are delay vs first seen."""
    by_slot: dict[int, list[Arrival]] = {}
    for a in arrivals:
        by_slot.setdefault(a.slot, []).append(a)
    delays: dict[str, list[int]] = {}
    endpoints = {a.endpoint for a in arrivals}
    if len(endpoints) < 2:
        raise ValueError("need at least two endpoints")
    for slot, rows in by_slot.items():
        present = {r.endpoint for r in rows}
        if present != endpoints:
            # incomplete slot — skip so we only compare shared events
            continue
        t0 = min(r.recv_us for r in rows)
        for r in rows:
            delays.setdefault(r.endpoint, []).append(r.recv_us - t0)
    if not delays:
        raise ValueError("no slot observed on every endpoint")
    return delays


def percentile_nearest_rank(sorted_samples: list[int], p: int) -> int:
    """Nearest-rank: index = ceil(p/100 * n) - 1. p in 1..100. Integer only."""
    if not sorted_samples:
        raise ValueError("empty sample")
    if p < 1 or p > 100:
        raise ValueError("percentile out of range")
    n = len(sorted_samples)
    idx = (p * n + 99) // 100 - 1
    if idx < 0:
        idx = 0
    return sorted_samples[idx]


def board(arrivals: list[Arrival], percents: tuple[int, ...] = (50, 90, 99)) -> dict:
    delays = relative_delays(arrivals)
    out = {}
    for ep, samples in delays.items():
        s = sorted(samples)
        out[ep] = {
            "n": len(s),
            **{f"p{p}": percentile_nearest_rank(s, p) for p in percents},
        }
    return {"unit": "microseconds", "method": "relative-to-first-seen, nearest-rank", "endpoints": out}


def arrivals_from_rows(rows: list[dict]) -> list[Arrival]:
    return [Arrival(slot=int(r["slot"]), endpoint=str(r["endpoint"]), recv_us=int(r["recv_us"])) for r in rows]
