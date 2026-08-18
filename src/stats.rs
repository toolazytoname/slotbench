//! Relative arrival percentiles. All times are integer microseconds.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Arrival {
    pub slot: u64,
    pub endpoint: String,
    pub recv_us: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EndpointStats {
    pub n: usize,
    pub p50: i64,
    pub p90: i64,
    pub p99: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Board {
    pub unit: &'static str,
    pub method: &'static str,
    pub endpoints: BTreeMap<String, EndpointStats>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StatsError {
    #[error("need at least two endpoints")]
    NeedTwoEndpoints,
    #[error("no slot observed on every endpoint")]
    NoSharedSlot,
    #[error("empty sample")]
    Empty,
    #[error("percentile out of range")]
    Percentile,
}

pub fn relative_delays(arrivals: &[Arrival]) -> Result<HashMap<String, Vec<i64>>, StatsError> {
    let endpoints: BTreeSet<&str> = arrivals.iter().map(|a| a.endpoint.as_str()).collect();
    if endpoints.len() < 2 {
        return Err(StatsError::NeedTwoEndpoints);
    }
    let mut by_slot: BTreeMap<u64, Vec<&Arrival>> = BTreeMap::new();
    for a in arrivals {
        by_slot.entry(a.slot).or_default().push(a);
    }
    let mut delays: HashMap<String, Vec<i64>> = HashMap::new();
    for rows in by_slot.values() {
        let present: BTreeSet<&str> = rows.iter().map(|r| r.endpoint.as_str()).collect();
        if present != endpoints {
            continue;
        }
        let t0 = rows.iter().map(|r| r.recv_us).min().unwrap();
        for r in rows {
            delays
                .entry(r.endpoint.clone())
                .or_default()
                .push(r.recv_us - t0);
        }
    }
    if delays.is_empty() {
        return Err(StatsError::NoSharedSlot);
    }
    Ok(delays)
}

/// Nearest-rank: index = ceil(p/100 * n) - 1.
pub fn percentile_nearest_rank(sorted: &[i64], p: u32) -> Result<i64, StatsError> {
    if sorted.is_empty() {
        return Err(StatsError::Empty);
    }
    if !(1..=100).contains(&p) {
        return Err(StatsError::Percentile);
    }
    let n = sorted.len();
    let idx = ((p as usize * n + 99) / 100).saturating_sub(1);
    Ok(sorted[idx])
}

pub fn board(arrivals: &[Arrival]) -> Result<Board, StatsError> {
    let delays = relative_delays(arrivals)?;
    let mut endpoints = BTreeMap::new();
    for (ep, mut samples) in delays {
        samples.sort_unstable();
        endpoints.insert(
            ep,
            EndpointStats {
                n: samples.len(),
                p50: percentile_nearest_rank(&samples, 50)?,
                p90: percentile_nearest_rank(&samples, 90)?,
                p99: percentile_nearest_rank(&samples, 99)?,
            },
        );
    }
    Ok(Board {
        unit: "microseconds",
        method: "relative-to-first-seen, nearest-rank",
        endpoints,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_known_list() {
        let s = [0, 10, 20, 30, 40];
        assert_eq!(percentile_nearest_rank(&s, 50).unwrap(), 20);
        assert_eq!(percentile_nearest_rank(&s, 100).unwrap(), 40);
    }

    #[test]
    fn relative_and_two_endpoints() {
        let arr = vec![
            Arrival { slot: 1, endpoint: "a".into(), recv_us: 1000 },
            Arrival { slot: 1, endpoint: "b".into(), recv_us: 1300 },
            Arrival { slot: 2, endpoint: "a".into(), recv_us: 2000 },
            Arrival { slot: 2, endpoint: "b".into(), recv_us: 2000 },
        ];
        let d = relative_delays(&arr).unwrap();
        assert_eq!(d["a"], vec![0, 0]);
        assert_eq!(d["b"], vec![300, 0]);
        assert_eq!(
            relative_delays(&[Arrival { slot: 1, endpoint: "a".into(), recv_us: 1 }]),
            Err(StatsError::NeedTwoEndpoints)
        );
    }

    #[test]
    fn board_fixture() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/arrivals.json");
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let arrivals: Vec<Arrival> = serde_json::from_value(v["arrivals"].clone()).unwrap();
        let b = board(&arrivals).unwrap();
        assert!(b.endpoints.contains_key("alpha"));
        assert!(b.endpoints.contains_key("beta"));
        assert_eq!(b.endpoints["alpha"].p50, 0);
    }
}
