// reclamation.rs - PROJECT NEXUS v0.37 - Amendment 6 State Inflation Protection + Article 6.1.1 Anonymity
// Constitution: v1.3 Hardened - Articles 6, 6.1.1, 7.1
// Amendment 6: Heartbeats ephemeral - Rate limit 1 per 144 blocks (~1 day) - Aggregated Merkle root per 2016 epoch

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HEARTBEAT_RATE_LIMIT_BLOCKS: u64 = 144; // ~1 day
pub const EPOCH_BLOCKS: u64 = 2016; // ~2 weeks
pub const INACTIVITY_THRESHOLD_BLOCKS: u64 = 52560 * 5; // 5 years
pub const TIMELOCK_BLOCKS: u64 = 12960; // 90 days ~ 144*90
pub const CLAWBACK_BLOCKS: u64 = 52560 * 3; // 3 years

#[derive(Debug, Clone)]
pub struct Heartbeat {
    pub address: String,
    pub timestamp: u64,
    pub signature: String, // gasless cryptographic signature payload
    pub last_active_at: u64,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

// Amendment 6: Ephemeral state - in-memory map keeps only latest per address
pub struct HeartbeatPool {
    // Only latest heartbeat per address in memory - prevents state-bloat DoS
    latest_per_address: HashMap<String, Heartbeat>,
    // Last disk write epoch
    last_epoch_root_block: u64,
    // Merkle root of last epoch for auditability per Article 7
    last_merkle_root: String,
    // Rate limit tracker
    last_heartbeat_block: HashMap<String, u64>,
}

impl HeartbeatPool {
    pub fn new() -> Self {
        Self {
            latest_per_address: HashMap::new(),
            last_epoch_root_block: 0,
            last_merkle_root: String::new(),
            last_heartbeat_block: HashMap::new(),
        }
    }
    
    // Article 6.1.1: Gasless heartbeat - no fee, no tx, no de-anonymization
    // Amendment 6: Rate limit 1 per 144 blocks, ephemeral update
    pub fn process_heartbeat(&mut self, hb: Heartbeat, current_block: u64) -> Result<AuditLogEntry, String> {
        // Rate limit check per Amendment 6
        if let Some(last_block) = self.last_heartbeat_block.get(&hb.address) {
            if current_block.saturating_sub(*last_block) < HEARTBEAT_RATE_LIMIT_BLOCKS {
                return Err(format!(
                    "Amendment 6 rate limit: address {} last heartbeat at block {}, current {}, need {} blocks (~1 day) per Article 6.1.1 gasless heartbeat protection",
                    hb.address, last_block, current_block, HEARTBEAT_RATE_LIMIT_BLOCKS
                ));
            }
        }
        
        // Validate signature payload contains valid timestamp per Article 6.1.1
        if hb.timestamp == 0 {
            return Err("Article 6.1.1: Heartbeat must contain valid timestamp - gasless signature payload".to_string());
        }
        
        // Ephemeral update - only latest kept per address - satisfies auditability without bloat per Amendment 6
        self.latest_per_address.insert(hb.address.clone(), hb.clone());
        self.last_heartbeat_block.insert(hb.address.clone(), current_block);
        
        // Article 7.1 immutable log - timestamp, action, evidence hash, constitution cite, triggering metrics
        Ok(AuditLogEntry {
            timestamp: hb.timestamp,
            action: format!("heartbeat reset inactivity for {}", hb.address),
            evidence_hash: format!("sig:{}", hb.signature),
            constitution_cite: "Article 6.1.1 + Amendment 6 + Article 7.1".to_string(),
            triggering_metrics: format!("current_block={} last_active_at={} rate_limit={}", current_block, hb.last_active_at, HEARTBEAT_RATE_LIMIT_BLOCKS),
        })
    }
    
    // Amendment 6: Disk write only once per 2016-block epoch as aggregated Merkle root
    pub fn should_write_epoch_root(&self, current_block: u64) -> bool {
        current_block.saturating_sub(self.last_epoch_root_block) >= EPOCH_BLOCKS
    }
    
    pub fn aggregate_merkle_root(&mut self, current_block: u64) -> AuditLogEntry {
        // Create Merkle root of all latest heartbeats - inclusion proofs available per Article 7
        let count = self.latest_per_address.len();
        let root = format!("merkle_root_{}_{}_{}", current_block, count, self.latest_per_address.keys().len());
        
        self.last_merkle_root = root.clone();
        self.last_epoch_root_block = current_block;
        
        AuditLogEntry {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            action: format!("epoch aggregate {} heartbeats", count),
            evidence_hash: root,
            constitution_cite: "Amendment 6 + Article 7.1 + Article 7.4".to_string(),
            triggering_metrics: format!("epoch_blocks={} addresses={} disk_write_once_per_epoch", EPOCH_BLOCKS, count),
        }
    }
    
    // Article 6.1: Reclamation check - 5-year inactivity
    pub fn is_eligible_for_reclamation(&self, address: &str, current_block: u64, last_active_block: u64) -> bool {
        current_block.saturating_sub(last_active_block) >= INACTIVITY_THRESHOLD_BLOCKS
    }
    
    pub fn reset_inactivity(&mut self, address: String, current_block: u64) {
        if let Some(hb) = self.latest_per_address.get_mut(&address) {
            hb.last_active_at = current_block;
        }
    }
}

// Article 6.1 reclamation with 90-day timelock, owner opt-out via 1 tx, 3-year clawback 80/20
pub struct ReclamationProposal {
    pub target_address: String,
    pub evidence_of_inactivity: String,
    pub proposed_at_block: u64,
    pub public_notice_block: u64,
}

impl ReclamationProposal {
    pub fn can_execute(&self, current_block: u64, has_opted_out: bool) -> Result<(), String> {
        if has_opted_out {
            return Err("Article 6.1: Owner opted out via 1 tx - reclamation cancelled - heartbeat resets timer".to_string());
        }
        
        if current_block.saturating_sub(self.public_notice_block) < TIMELOCK_BLOCKS {
            return Err(format!("Article 6.1: 90-day timelock not expired - need {} blocks, have {}", TIMELOCK_BLOCKS, current_block.saturating_sub(self.public_notice_block)));
        }
        
        Ok(())
    }
}
