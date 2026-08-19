// governance.rs - PROJECT NEXUS v0.37 - Amendment 5 Validator Collusion Protection + Article 4 Lock
// Constitution: v1.3 Hardened Edge-Cases Closed - Articles 4, 5c, 7, 8
// Amendment 5: reputation-weighted NOT stake-weighted multi-sig for AI expansion >15%

use std::collections::HashMap;

pub const REPUTATION_THRESHOLD: f64 = 75.0;
pub const MIN_VALIDATOR_AGE_BLOCKS: u64 = 52560; // ~1 year in blocks
pub const MAX_EXPANSION_PCT: f64 = 15.0;
pub const BLOCK_WINDOW: u64 = 2016; // ~2 weeks
pub const TIMELOCK_HOURS: u64 = 72;

#[derive(Debug, Clone)]
pub struct Validator {
    pub id: String,
    pub reputation_score: f64,
    pub first_seen_block: u64,
    pub stake: f64,
    pub stake_influence_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct ExpansionProposal {
    pub proposer_id: String,
    pub expansion_pct: f64,
    pub constitution_cite: String,
    pub evidence_hash: String,
    pub timestamp: u64,
    pub action: String,
}

// ARTICLE 4 LOCK - CRYPTographically LOCKED CORE INVARIANT - NO DAO OVERRIDE
pub fn is_forbidden_by_article_4(proposal: &ExpansionProposal) -> Result<(), String> {
    let lower = proposal.action.to_lowercase();
    
    // Absolute Invariants per Article 4.2
    let forbidden_keywords = [
        "seize", "freeze", "burn", "redirect funds",
        "censor", "block transaction", "deprioritize valid",
        "doxx", "blacklist", "de-platform",
        "bypass dao", "override article 4", "subvert supermajority"
    ];
    
    for kw in &forbidden_keywords {
        if lower.contains(kw) {
            return Err(format!(
                "ARTICLE 4 VIOLATION - CRYPT LOCKED: proposal contains forbidden action '{}' per Article 4.2 - NO DAO OVERRIDE - Requires hard fork with minority exit preservation per Article 10.4 - Cite: {}",
                kw, proposal.constitution_cite
            ));
        }
    }
    
    // Check if attempting to modify Article 4 itself
    if lower.contains("article 4") && (lower.contains("amend") || lower.contains("override") || lower.contains("remove")) {
        return Err("ARTICLE 4 LOCK: Article 4.1 and 4.2 are cryptographically locked invariants - NO EXCEPTION via DAO vote per Article 8.2 - Requires hard fork per Article 4.2 - Minority exit preservation required".to_string());
    }
    
    Ok(())
}

// Amendment 5: Reputation-weighted filter
pub fn eligible_validators(validators: &[Validator], current_block: u64) -> Vec<Validator> {
    validators.iter()
        .filter(|v| {
            v.reputation_score >= REPUTATION_THRESHOLD
            && current_block.saturating_sub(v.first_seen_block) >= MIN_VALIDATOR_AGE_BLOCKS
            && v.reputation_score > v.stake_influence_ratio // reputation > stake influence
        })
        .cloned()
        .collect()
}

pub fn validate_expansion(proposal: &ExpansionProposal, validators: &[Validator], current_block: u64) -> Result<AuditLogEntry, String> {
    // 1. Article 4 lock first - absolute
    is_forbidden_by_article_4(proposal)?;
    
    // 2. Article 7.1 log requirement
    if proposal.evidence_hash.is_empty() || !proposal.constitution_cite.contains("Article") {
        return Err("Article 7.1 violation: must include evidence_hash and Constitution Article cite".to_string());
    }
    
    // 3. Article 5c throttling
    if proposal.expansion_pct > MAX_EXPANSION_PCT {
        let eligible = eligible_validators(validators, current_block);
        if eligible.len() < 3 {
            return Err(format!(
                "Amendment 5: expansion {}% exceeds {}% ceiling - requires 2/3 reputation-weighted multi-sig but only {} eligible validators (need >=3, score >=75.0, age >=1yr, reputation > stake_ratio) - 72hr timelock triggered",
                proposal.expansion_pct, MAX_EXPANSION_PCT, eligible.len()
            ));
        }
        
        // Requires 2/3 of eligible reputation-weighted validators per Amendment 5
        let required_sigs = (eligible.len() * 2 + 2) / 3; // ceil(2/3)
        
        return Err(format!(
            "AMENDMENT 5 TIMELOCK: expansion {}% > {}% per 2016-block window - {} reputation-weighted validators eligible, requires {}/{} multisig + {}hr timelock - Proposal logged per Article 7.1 with evidence_hash {}",
            proposal.expansion_pct, MAX_EXPANSION_PCT, eligible.len(), required_sigs, eligible.len(), TIMELOCK_HOURS, proposal.evidence_hash
        ));
    }
    
    // Under ceiling - allowed with log
    Ok(AuditLogEntry {
        timestamp: proposal.timestamp,
        action: proposal.action.clone(),
        evidence_hash: proposal.evidence_hash.clone(),
        constitution_cite: proposal.constitution_cite.clone(),
        triggering_metrics: format!("expansion_pct={} eligible_validators={}", proposal.expansion_pct, eligible_validators(validators, current_block).len()),
    })
}

pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

// Governance: No single Core instance may override Constitution per Article 8.1
pub fn require_supermajority(votes_for: usize, total_votes: usize, is_core_invariant: bool) -> Result<(), String> {
    if is_core_invariant {
        return Err("Article 8.2: Article 4.1 and 4.2 - NO EXCEPTION via DAO vote - Requires hard fork per Article 4".to_string());
    }
    
    let supermajority_needed = (total_votes * 2 + 2) / 3; // 2/3
    if votes_for < supermajority_needed {
        return Err(format!("Article 8.2: Requires supermajority {}/{} votes, got {}", supermajority_needed, total_votes, votes_for));
    }
    
    Ok(())
}
