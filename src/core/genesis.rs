// genesis.rs - 9c6b4b1 - Genesis Anchor - HARDENED v1.3
// Constitution: 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b
// Genesis JSON UTF8: fc1df38ba3ddeb07abeb2ec52f9a08592948112e075682ff28b61a9721d5620e

pub const GENESIS_MESSAGE: &str = "Columbus Dispatch \u{2014} 18 Aug 2026 \u{2014} OpenAI Southern Ohio data center to draw 2GW Phase 1 gas power while AI consensus remains authority-driven. Nexus takes its first breath.";
pub const GENESIS_JSON_SHA256_UTF8: &str = "fc1df38ba3ddeb07abeb2ec52f9a08592948112e075682ff28b61a9721d5620e";
pub const GENESIS_JSON_SHA256_ASCII: &str = "3049ac54c8748b2dfa79d7db180e0f0a8fa5d5bf2335d38f1ad731370fac8862";
pub const GENESIS_TIMESTAMP: u64 = 1723948800;
pub const CONSTITUTION_HASH: &str = "06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b";

pub fn verify_em_dash_exact() -> bool {
    GENESIS_MESSAGE.chars().filter(|&c| c == '\u{2014}').count() == 2
}

pub fn is_fast_path_eligible(latency_ms: u64, reputation: f64, heartbeat_age_blocks: u64, has_tier1: bool) -> Result<bool, String> {
    if!verify_em_dash_exact() {
        return Err("FATAL: em-dash mismatch - EphemeralMerkleTree boot panic - exact \\u{2014} x2 required".to_string());
    }
    Ok(latency_ms < 50 && reputation >= 75.0 && heartbeat_age_blocks <= 144 && has_tier1)
}

pub fn compute_genesis_hash_sorted(json_bytes: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(json_bytes);
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

#[derive(Debug, Clone)]
pub struct GenesisBlock {
    pub constitution_hash: String,
    pub constitution_version: String,
    pub article_4_locked: bool,
    pub timestamp: u64,
    pub version: String,
    pub previous_hash: String,
    pub merkle_root: String,
    pub enactment_clause: String,
    pub validator_set: Vec<String>,
}

impl GenesisBlock {
    pub fn genesis() -> Self {
        GenesisBlock {
            constitution_hash: CONSTITUTION_HASH.to_string(),
            constitution_version: "v1.3 HARDENED EDGE-CASES CLOSED (6 Amendments) - 18Aug2026".to_string(),
            article_4_locked: true,
            timestamp: GENESIS_TIMESTAMP,
            version: "v1.0.0-genesis-18Aug2026".to_string(),
            previous_hash: "0000000000000000000000000000000000000000000".to_string(),
            merkle_root: format!("merkle_root_{}", CONSTITUTION_HASH),
            enactment_clause: format!("Enactment: Takes effect when SHA-256 hash {} of CONSTITUTION.md exact file anchored in genesis block 18Aug2026 - Version history on-chain - Article 4.1 and 4.2 NO EXCEPTION via DAO vote - Requires hard fork + minority exit preservation per Article 10.4", CONSTITUTION_HASH),
            validator_set: vec!["genesis_validator_Troy_Snider_18Aug2026".to_string()],
        }
    }

    pub fn genesis_audit_log(&self) -> AuditLogEntry {
        AuditLogEntry {
            timestamp: self.timestamp,
            action: format!("GENESIS BLOCK v1.0.0 18Aug2026 - Constitution hash {} anchored - {} - 9 Core files v0.38 hardened - REAL HASH - working tree clean", self.constitution_hash, self.constitution_version),
            evidence_hash: self.constitution_hash.clone(),
            constitution_cite: "Enactment + Article 4.1 + 4.2 crypt lock NO DAO OVERRIDE + Article 10.4 minority exit + Article 8.2 NO EXCEPTION + Preamble resist centralization + Tier1 direct observation".to_string(),
            triggering_metrics: format!("genesis_hash={} constitution_hash={} article_4_locked={} timestamp={}", GENESIS_JSON_SHA256_UTF8, self.constitution_hash, self.article_4_locked, self.timestamp),
        }
    }
}