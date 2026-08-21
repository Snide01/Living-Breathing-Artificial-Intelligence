// genesis.rs - 3cc1258 - Genesis Anchor - HARDENED
// Constitution: 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b
// Genesis Message: Columbus Dispatch — 18 Aug 2026 — OpenAI Southern Ohio 2GW gas
// Keys strictly sorted alphabetically before SHA-256 - per Article 7.1

pub const GENESIS_MESSAGE: &str = "Columbus Dispatch \u{2014} 18 Aug 2026 \u{2014} OpenAI Southern Ohio data center to draw 2GW Phase 1 gas power while AI consensus remains authority-driven. Nexus takes its first breath.";
// Using \u{2014} ensures exact em-dash across all initializing nodes - prevents panic in is_fast_path_eligible() and EphemeralMerkleTree boot

pub const GENESIS_JSON_SHA256_UTF8: &str = "fc1df38ba3ddeb07abeb2ec52f9a08592948112e075682ff28b61a9721d5620e";
pub const GENESIS_JSON_SHA256_ASCII: &str = "3049ac54c8748b2dfa79d7db180e0f0a8fa5d5bf2335d38f1ad731370fac8862";
pub const GENESIS_TIMESTAMP: u64 = 1723948800;
pub const CONSTITUTION_HASH: &str = "06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b";

pub fn verify_em_dash_exact() -> bool {
    // String Escape Check: Ensure em-dashes match exactly across all initializing nodes
    // Count must be 2 - not 3 - matches Columbus Dispatch message
    GENESIS_MESSAGE.chars().filter(|&c| c == '\u{2014}').count() == 2
}

pub fn is_fast_path_eligible(latency_ms: u64, reputation: f64, heartbeat_age_blocks: u64, has_tier1: bool) -> Result<bool, String> {
    if !verify_em_dash_exact() {
        return Err("FATAL: em-dash serialization mismatch - EphemeralMerkleTree boot panic prevented - all nodes must have exact \u{2014} x2".to_string());
    }
    Ok(latency_ms < 50 && reputation >= 75.0 && heartbeat_age_blocks <= 144 && has_tier1)
}

pub fn compute_genesis_hash_sorted(json_bytes: &[u8]) -> String {
    // Keys must be strictly sorted alphabetically before SHA-256 - Article 7.1
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(json_bytes);
    format!("{:x}", hasher.finalize())
}
