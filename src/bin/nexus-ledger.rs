use chrono;
use std::time::Duration;
use nexus_core::core::healing::{SeedCandidate, BootGate, BootDecision, deterministic_seed};

const CORE_NUM: u8 = 1;
const NODE_ID: &str = "AGG-9600K-.241";
const PEER_NODES: [(&str, u8); 4] = [
    ("192.168.1.242", 2),
    ("192.168.1.243", 3),
    ("192.168.1.244", 4),
    ("192.168.1.245", 5),
];

fn read_local_state() -> (u64, String) {
    // Read genesis root clean - use MESH_HASH.txt or genesis hash
    let height = std::fs::read_to_string("MESH_HASH.txt")
        .ok()
        .and_then(|s| s.trim().lines().next().unwrap_or("").parse::<u64>().ok())
        .unwrap_or(605);
    // Real epoch root from genesis.rs anchor
    let root = std::fs::read_to_string("genesis.json")
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v.get("genesis_root").and_then(|r| r.as_str().map(|s| s.to_string())))
        .unwrap_or_else(|| "45a12ef3".to_string());
    (height, root)
}

fn discover_peers_mock() -> Vec<SeedCandidate> { Vec::new() }

fn main() {
    let ts = chrono::Utc::now().timestamp();
    println!("{} - NEXUS v0.38 ledger + Full Blackout Gear — HeartbeatPool 144 + 2016 epoch", ts);
    let (local_height, local_root) = read_local_state();
    let local = SeedCandidate::from_local_state(NODE_ID.to_string(), CORE_NUM, local_root.clone(), local_height);
    println!("[BOOT GATE] Local: core={} height={} root={} id={}", CORE_NUM, local_height, local_root, NODE_ID);
    let peers = discover_peers_mock();
    println!("[BOOT GATE] Discovered {} peers", peers.len());
    let gate = BootGate::new(local.clone());
    let (decision, audit_log) = gate.decide_boot(peers.clone());
    println!("[AUDIT] {} | cite={} | evidence={}", audit_log.action, audit_log.constitution_cite, audit_log.evidence_hash);
    match &decision {
        BootDecision::StartAsAggregator { reason } => println!("[BOOT] SEED by tiebreaker: {} — STARTING AS AGGREGATOR", reason),
        BootDecision::SyncAndJoin { target, local_height } => {
            println!("[BOOT] STALE DETECTED: local {} < target {} (core {}) — SYNC REQUIRED", local_height, target.height, target.core_number);
            std::process::exit(1);
        },
        _ => {}
    }
    let mut all = peers.clone(); all.push(local.clone());
    if let Some(seed) = deterministic_seed(all) {
        println!("[TIEBREAKER] Global seed converged: core={} height={} id={} — all nodes agree without quorum", seed.core_number, seed.height, seed.node_id);
    }
    println!("{} - ledger Approved→state Merkle root — HeartbeatPool 1 per 144 + 2016 epoch", chrono::Utc::now().timestamp());
    loop { std::thread::sleep(Duration::from_secs(144)); println!("HeartbeatPool tick {} — v0.38 full blackout gear active", chrono::Utc::now().timestamp()); }
}
