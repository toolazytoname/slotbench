//! Sample getSlot on two or more JSON-RPC endpoints; pair by slot number.

use crate::stats::Arrival;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Error)]
pub enum LiveError {
    #[error("need at least two endpoints")]
    NeedTwo,
    #[error("rpc {0}: {1}")]
    Rpc(String, String),
}

pub fn get_slot(url: &str) -> Result<u64, String> {
    let body = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"getSlot"});
    let resp = ureq::post(url)
        .set("Content-Type", "application/json")
        .set("User-Agent", "slotbench/0.1")
        .send_json(body)
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    v.get("result")
        .and_then(|r| r.as_u64())
        .ok_or_else(|| format!("bad getSlot: {v}"))
}

/// Poll until `samples` slots have been seen on every endpoint.
pub fn sample(endpoints: &[Endpoint], samples: usize) -> Result<Vec<Arrival>, LiveError> {
    if endpoints.len() < 2 {
        return Err(LiveError::NeedTwo);
    }
    let start = Instant::now();
    let mut first: BTreeMap<(u64, String), i64> = BTreeMap::new();
    let names: BTreeSet<String> = endpoints.iter().map(|e| e.name.clone()).collect();
    let mut complete: BTreeSet<u64> = BTreeSet::new();
    let mut spins = 0;
    while complete.len() < samples && spins < samples * 40 + 20 {
        spins += 1;
        for ep in endpoints {
            let slot = get_slot(&ep.url).map_err(|e| LiveError::Rpc(ep.name.clone(), e))?;
            let recv_us = start.elapsed().as_micros() as i64;
            first.entry((slot, ep.name.clone())).or_insert(recv_us);
        }
        let slots: BTreeSet<u64> = first.keys().map(|(s, _)| *s).collect();
        for slot in slots {
            let have: BTreeSet<String> = first
                .keys()
                .filter(|(s, _)| *s == slot)
                .map(|(_, n)| n.clone())
                .collect();
            if have == names {
                complete.insert(slot);
            }
        }
        if complete.len() < samples {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
    let mut arrivals = Vec::new();
    for slot in complete.iter().take(samples) {
        for ep in endpoints {
            if let Some(us) = first.get(&(*slot, ep.name.clone())) {
                arrivals.push(Arrival {
                    slot: *slot,
                    endpoint: ep.name.clone(),
                    recv_us: *us,
                });
            }
        }
    }
    if arrivals.is_empty() {
        return Err(LiveError::Rpc("*".into(), "no shared slot observed".into()));
    }
    Ok(arrivals)
}
