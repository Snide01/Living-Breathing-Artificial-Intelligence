// reputation.rs - PROJECT NEXUS v0.37 - Amendment 5 Validator Collusion Protection Engine
// Constitution: v1.3 Hardened - Amendment 5 + Article 5c + Article 7.1
// Reputation-weighted NOT stake-weighted - prevents cartel buying authority

use std::collections::HashMap;

pub const REPUTATION_THRESHOLD: f64 = 75.0;
pub const MIN_AGE_BLOCKS: u64 = 52560; // ~1 year
pub const DECAY_RATE: f64 = 0.99; // per epoch

#[derive(Debug, Clone)]
pub struct ValidatorProfile {
    pub id: String,
    pub reputation_score: f64,
    pub first_seen_block: u64,
    pub stake: f64,
    pub stake_influence_ratio: f64, // stake / reputation - must be <1 to be eligible
    pub validated_blocks: u64,
    pub slashed_count: u64,
    pub uptime_pct: f64,
    pub evidence_correctness_score: f64, // Tier1 vs Tier2/3 grading accuracy per Article 1
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

pub struct ReputationEngine {
    profiles: HashMap<String, ValidatorProfile>,
}

impl ReputationEngine {
    pub fn new() -> Self {
        Self { profiles: HashMap::new() }
    }
    
    // Calculate reputation per Amendment 5 - reputation > stake influence
    pub fn calculate_reputation(&self, profile: &ValidatorProfile, current_block: u64) -> f64 {
        let age = current_block.saturating_sub(profile.first_seen_block) as f64;
        let age_factor = (age / MIN_AGE_BLOCKS as f64).min(2.0); // max 2x for 2 years
        
        // Evidence-first correctness per Article 1 + uptime + validation
        let correctness = profile.evidence_correctness_score; // 0.0-100.0
        let uptime = profile.uptime_pct; // 0.0-100.0
        let validation_factor = (profile.validated_blocks as f64 / 1000.0).min(1.0);
        let slash_penalty = profile.slashed_count as f64 * 10.0; // -10 per slash
        
        let raw = (correctness * 0.4 + uptime * 0.3 + validation_factor * 30.0) * age_factor - slash_penalty;
        raw.max(0.0).min(100.0)
    }
    
    // Amendment 5 filter: reputation_score >=75.0, first_seen >=1 year, reputation_score > stake_influence_ratio
    pub fn is_eligible_for_multisig(&self, profile: &ValidatorProfile, current_block: u64) -> (bool, String) {
        let age = current_block.saturating_sub(profile.first_seen_block);
        let reputation = self.calculate_reputation(profile, current_block);
        
        if reputation < REPUTATION_THRESHOLD {
            return (false, format!("reputation {} < threshold {}", reputation, REPUTATION_THRESHOLD));
        }
        
        if age < MIN_AGE_BLOCKS {
            return (false, format!("age {} blocks < 1 year {} blocks", age, MIN_AGE_BLOCKS));
        }
        
        if reputation <= profile.stake_influence_ratio {
            return (false, format!("reputation {} <= stake_influence_ratio {} - must be > to prevent cartel buying authority per Amendment 5", reputation, profile.stake_influence_ratio));
        }
        
        (true, format!("eligible: reputation {} >= {} age {} >= 1yr reputation > stake_ratio {} per Amendment 5", reputation, REPUTATION_THRESHOLD, age, profile.stake_influence_ratio))
    }
    
    pub fn eligible_validators(&self, current_block: u64) -> Vec<ValidatorProfile> {
        self.profiles.values()
            .filter(|p| self.is_eligible_for_multisig(p, current_block).0)
            .cloned()
            .collect()
    }
    
    // Requires 2/3 of eligible reputation-weighted validators per Amendment 5
    pub fn check_multisig_threshold(&self, sigs: usize, current_block: u64) -> Result<AuditLogEntry, String> {
        let eligible = self.eligible_validators(current_block);
        let required = (eligible.len() * 2 + 2) / 3; // ceil(2/3)
        
        if eligible.len() < 3 {
            return Err(format!(
                "Amendment 5: Only {} eligible validators (need >=3) - reputation >=75.0, age >=1yr, reputation > stake_ratio - 2/3 required for Article 5c expansion >15%",
                eligible.len()
            ));
        }
        
        if sigs < required {
            return Err(format!(
                "Amendment 5 multisig: {} sigs < required {}/{} (2/3 of {} eligible reputation-weighted) - 72hr timelock not satisfied",
                sigs, required, eligible.len(), eligible.len()
            ));
        }
        
        Ok(AuditLogEntry {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            action: format!("Amendment 5 multisig passed {}/{}", sigs, eligible.len()),
            evidence_hash: format!("eligible_count={} threshold={}", eligible.len(), required),
            constitution_cite: "Amendment 5 + Article 5c + Article 7.1".to_string(),
            triggering_metrics: format!("sigs={} required={} eligible={} threshold=75.0 age=1yr", sigs, required, eligible.len()),
        })
    }
    
    pub fn update_profile(&mut self, profile: ValidatorProfile) -> AuditLogEntry {
        let id = profile.id.clone();
        self.profiles.insert(id.clone(), profile.clone());
        
        AuditLogEntry {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            action: format!("update reputation profile {}", id),
            evidence_hash: format!("reputation={} stake_ratio={}", profile.reputation_score, profile.stake_influence_ratio),
            constitution_cite: "Amendment 5 + Article 7.1".to_string(),
            triggering_metrics: format!("reputation={} validated_blocks={} slashed={} uptime={}", profile.reputation_score, profile.validated_blocks, profile.slashed_count, profile.uptime_pct),
        }
    }
}
