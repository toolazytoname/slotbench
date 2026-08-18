use std::path::PathBuf;
use std::process::Command;

fn exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_slotbench"))
}
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join(name)
}

#[test]
fn bench_twice_same() {
    let a = Command::new(exe())
        .args(["bench", "--fixture", fixture("arrivals.json").to_str().unwrap()])
        .output()
        .unwrap();
    let b = Command::new(exe())
        .args(["bench", "--fixture", fixture("arrivals.json").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
    assert!(b.status.success());
    let va: serde_json::Value = serde_json::from_slice(&a.stdout).unwrap();
    let vb: serde_json::Value = serde_json::from_slice(&b.stdout).unwrap();
    assert_eq!(va, vb);
    assert!(va["endpoints"]["beta"].get("p50").is_some());
    assert!(va["endpoints"]["beta"].get("p99").is_some());
}

#[test]
fn one_endpoint_fails() {
    let out = Command::new(exe())
        .args(["bench", "--fixture", fixture("one_endpoint.json").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success());
}

#[test]
fn doctor_secret() {
    let out = Command::new(exe())
        .args(["doctor", "--config", fixture("config.secret.json").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let s = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(!s.contains("PLANT-SECRET-DO-NOT-LOG"));
}
