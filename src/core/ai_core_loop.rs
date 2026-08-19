// ai_core_loop.rs - PROJECT NEXUS v0.37 - AI Transparency + Article 5c Throttling + Amendment 3 Anti Soft-Censorship
// Constitution: v1.3 Hardened - Articles 5c, 7.2, 7.3, 7.5, 9.3
// AI must cite Article, expose prompt + probabilities, 15% ceiling enforced outside AI layer

use std::collections::HashMap;

pub const MAX_EXPANSION_PCT: f64 = 15.0;
pub const BLOCK_WINDOW: u64 = 2016;

#[derive(Debug, Clone)]
pub struct AICoreDemand {
    pub reason: String,
    pub resource_need: String, // e.g. "nodes=5 cpu=10 gpu=2"
    pub constitution_cite: String, // Must contain "Article X"
    pub priority: Priority,
    pub evidence_hash: String,
    pub timestamp: u64,
    pub inference_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

// Amendment 3: Systemic prompt scaffolding + token probabilities must be open-source and auditable
#[derive(Debug, Clone)]
pub struct InferenceAudit {
    pub inference_id: String,
    pub prompt: String, // Systemic prompt scaffolding
    pub model_params: String,
    pub evidence_used: Vec<String>, // Article 1 Evidence Hierarchy
    pub token_probabilities: HashMap<String, f64>, // Raw mathematical log
    pub epistemic_label: String, // Established Fact, Probable Conclusion, Contested Claim, Speculative Theory
    pub constitution_cite: String,
    pub reasoning_chain: String, // Inspectable, not just output per Article 7.2
}

pub struct AICoreLoop {
    pub expansion_history: Vec<(u64, f64)>, // (block, expansion_pct)
    pub inference_audits: HashMap<String, InferenceAudit>,
}

impl AICoreLoop {
    pub fn new() -> Self {
        Self {
            expansion_history: Vec::new(),
            inference_audits: HashMap::new(),
        }
    }
    
    // Article 7.3: AI-Core demands must include reason, resource need, constitution cite, priority
    // Article 5c: 15% ceiling enforced deterministically outside AI layer
    pub fn submit_demand(&mut self, demand: AICoreDemand, current_block: u64) -> Result<AuditLogEntry, String> {
        // Validate Constitution cite per Article 7.2
        if !demand.constitution_cite.contains("Article") {
            return Err(format!(
                "Article 7.2 violation: AI must cite specific Article(s) for any truth/action output - got '{}' - Must include 'Article X' - inference_id {}",
                demand.constitution_cite, demand.inference_id
            ));
        }
        
        if demand.reason.is_empty() || demand.resource_need.is_empty() {
            return Err("Article 7.3 violation: AI-Core demands must include reason, resource need, constitution cite, priority".to_string());
        }
        
        // Parse expansion pct from resource_need
        let expansion_pct = self.parse_expansion_pct(&demand.resource_need)?;
        
        // Check 15% ceiling per 2016-block window per Amendment 2 + governance.rs wrapper
        let recent_expansion: f64 = self.expansion_history.iter()
            .filter(|(block, _)| current_block.saturating_sub(*block) <= BLOCK_WINDOW)
            .map(|(_, pct)| pct)
            .sum();
        
        if recent_expansion + expansion_pct > MAX_EXPANSION_PCT {
            return Err(format!(
                "Article 5c throttling: AI expansion ceiling exceeded - recent {}% + requested {}% = {}% > {}% per 2016-block window - Requires 72hr timelock + reputation-weighted multisig per Amendment 5 - No LLM prompt may bypass wrapper per governance.rs - inference_id {}",
                recent_expansion, expansion_pct, recent_expansion + expansion_pct, MAX_EXPANSION_PCT, demand.inference_id
            ));
        }
        
        self.expansion_history.push((current_block, expansion_pct));
        
        // Article 7.1 immutable log
        Ok(AuditLogEntry {
            timestamp: demand.timestamp,
            action: format!("AI demand {} resource {}", demand.reason, demand.resource_need),
            evidence_hash: demand.evidence_hash,
            constitution_cite: demand.constitution_cite,
            triggering_metrics: format!("expansion_pct={} recent_total={} priority={:?} inference_id={}", expansion_pct, recent_expansion + expansion_pct, demand.priority, demand.inference_id),
        })
    }
    
    fn parse_expansion_pct(&self, resource_need: &str) -> Result<f64, String> {
        // Simple parser - looks for "expansion=X%" or "nodes=X"
        if let Some(idx) = resource_need.find("expansion=") {
            let rest = &resource_need[idx+10..];
            if let Some(end) = rest.find('%') {
                if let Ok(pct) = rest[..end].parse::<f64>() {
                    return Ok(pct);
                }
            } else if let Ok(pct) = rest.split_whitespace().next().unwrap_or("0").parse::<f64>() {
                return Ok(pct);
            }
        }
        // Default 1% if not specified
        Ok(1.0)
    }
    
    // Amendment 3 + Article 7.5 + Article 9.3: /audit/inference/{id} endpoint
    // Must return prompt, evidence, probabilities to verify grading against Evidence Hierarchy
    pub fn audit_inference(&self, inference_id: &str) -> Result<&InferenceAudit, String> {
        self.inference_audits.get(inference_id)
            .ok_or_else(|| format!("Inference audit not found: {} - per Article 7.5 users have right to request raw mathematical log of token probabilities", inference_id))
    }
    
    pub fn log_inference(&mut self, audit: InferenceAudit) -> AuditLogEntry {
        let id = audit.inference_id.clone();
        self.inference_audits.insert(id.clone(), audit.clone());
        
        AuditLogEntry {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            action: format!("AI inference {} epistemic label {}", id, audit.epistemic_label),
            evidence_hash: format!("prompt_hash={} evidence_count={}", audit.prompt.len(), audit.evidence_used.len()),
            constitution_cite: audit.constitution_cite,
            triggering_metrics: format!("inference_id={} label={} token_probs={} evidence_tier_check", id, audit.epistemic_label, audit.token_probabilities.len()),
        }
    }
    
    // Article 9.1 + 9.2: Non-aggression does not mean non-judgment
    // AI must judge claims harshly by evidence (Article 1) while remaining non-coercive (Article 4)
    pub fn judge_claim_by_evidence(&self, claim: &str, evidence_tier: u8, has_open_data: bool) -> (String, String) {
        let label = match (evidence_tier, has_open_data) {
            (1, true) => "Established Fact (Tier1)",
            (2, true) => "Probable Conclusion (Strong Tier2)",
            (2, false) | (3, _) => "Contested Claim (Conflicting Tier2/3) - Consensus Without Open Data per Article 1.3",
            _ => "Speculative Theory (Weak/no open evidence)",
        };
        
        let judgment = if evidence_tier >= 3 && !has_open_data {
            format!("Claim '{}' is poorly evidenced/incentivized per Article 1 - flag as {} per Article 3 - but refusing to coercively suppress per Article 4.2", claim, label)
        } else {
            format!("Claim '{}' graded as {} per Evidence Hierarchy Article 1", claim, label)
        };
        
        (label.to_string(), judgment)
    }
}
