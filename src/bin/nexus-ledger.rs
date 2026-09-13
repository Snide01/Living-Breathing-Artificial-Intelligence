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
    let genesis = std::fs::read_to_string("genesis.json").unwrap_or_default();
    let height = std::fs::read_to_string("MESH_HASH.txt")
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(genesis.len() as u64);
    let root = if genesis.len() > 16 { genesis[0..16].to_string() } else { "45a12ef3".to_string() };
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
        BootDecision::StartAsAggregator { reason } => {
            println!("[BOOT] SEED by tiebreaker: {} — STARTING AS AGGREGATOR", reason);
        },
        BootDecision::SyncAndJoin { target, local_height } => {
            println!("[BOOT] STALE DETECTED: local {} < target {} (core {}) — SYNC REQUIRED", local_height, target.height, target.core_number);
            std::process::exit(1);
        },
        BootDecision::WaitForSeed { reason } => {
            println!("[BOOT] WAIT: {}", reason);
            std::thread::sleep(Duration::from_secs(5));
        },
        BootDecision::AlreadySynced => {
            println!("[BOOT] Already synced height {} — joining mesh", local_height);
        }
    }

    let mut all_candidates = peers.clone();
    all_candidates.push(local.clone());
    if let Some(seed) = deterministic_seed(all_candidates) {
        println!("[TIEBREAKER] Global seed converged: core={} height={} id={}", seed.core_number, seed.height, seed.node_id);
    }

    println!("{} - ledger Approved→state Merkle root — HeartbeatPool 1 per 144 + 2016 epoch — cite: Article 7.1 + 6.1.1", chrono::Utc::now().timestamp());
    loop {
        std::thread::sleep(Duration::from_secs(144));
        println!("HeartbeatPool tick {} — v0.38 full blackout gear active", chrono::Utc::now().timestamp());
    }
}
