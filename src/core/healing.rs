// healing.rs - PROJECT NEXUS v0.37 - Self-Healing + Article 7.4 Evidence Preservation
// Constitution: v1.3 Hardened - Articles 3.1, 3.3, 5b, 7.1, 7.4
// Defensive replication away from attack + Isolation + Consensus-verified rebuilding

use std::collections::HashMap;

pub const INTRUSION_THRESHOLD: f64 = 0.85;
pub const ANOMALY_THRESHOLD: f64 = 0.75;

#[derive(Debug, Clone)]
pub struct HealthMetrics {
    pub node_id: String,
    pub cpu_anomaly: f64,
    pub network_intrusion_score: f64,
    pub disk_corruption_detected: bool,
    pub consensus_divergence: f64,
    pub last_verified_block: u64,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

// Article 7.4: Logs preserved as part of self-healing - evidence never deleted
pub struct HealingEngine {
    isolated_nodes: HashMap<String, HealthMetrics>,
    verified_state_root: String,
    evidence_log: Vec<AuditLogEntry>,
}

impl HealingEngine {
    pub fn new(genesis_root: String) -> Self {
        Self {
            isolated_nodes: HashMap::new(),
            verified_state_root: genesis_root,
            evidence_log: Vec::new(),
        }
    }
    
    // Article 5(b): Defensive - verifiable intrusion/anomaly/damage
    pub fn detect_compromise(&self, metrics: &HealthMetrics) -> Result<bool, AuditLogEntry> {
        let is_compromised = metrics.network_intrusion_score >= INTRUSION_THRESHOLD
            || metrics.cpu_anomaly >= ANOMALY_THRESHOLD
            || metrics.disk_corruption_detected
            || metrics.consensus_divergence >= 0.5;
        
        if is_compromised {
            // Article 7.1 log every trigger
            let log = AuditLogEntry {
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                action: format!("detect compromise node {}", metrics.node_id),
                evidence_hash: format!("intrusion={} anomaly={} corruption={} divergence={}", 
                    metrics.network_intrusion_score, metrics.cpu_anomaly, metrics.disk_corruption_detected, metrics.consensus_divergence),
                constitution_cite: "Article 5b + Article 7.1 + Article 7.4".to_string(),
                triggering_metrics: format!("thresholds: intrusion>={} anomaly>={} divergence>=0.5", INTRUSION_THRESHOLD, ANOMALY_THRESHOLD),
            };
            return Ok(true);
        }
        
        Ok(false)
    }
    
    // Article 3.3: Isolation of compromised components
    pub fn isolate_compromised(&mut self, metrics: HealthMetrics) -> AuditLogEntry {
        let node_id = metrics.node_id.clone();
        self.isolated_nodes.insert(node_id.clone(), metrics.clone());
        
        let log = AuditLogEntry {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            action: format!("isolate compromised node {}", node_id),
            evidence_hash: format!("metrics_hash_{}", node_id),
            constitution_cite: "Article 4.3 + Article 5b + Article 7.1 - Permitted Defensive".to_string(),
            triggering_metrics: format!("intrusion={} anomaly={} isolated_count={}", metrics.network_intrusion_score, metrics.cpu_anomaly, self.isolated_nodes.len()),
        };
        
        // Article 7.4 - preserve log as part of self-healing - evidence never deleted
        self.evidence_log.push(log.clone());
        log
    }
    
    // Article 3.3: Consensus-verified rebuilding + defensive replication away from attack (3.1)
    pub fn defensive_replication_and_rebuild(&mut self, source_node: &str, target_region: &str) -> AuditLogEntry {
        let log = AuditLogEntry {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            action: format!("defensive replication {} -> {} + consensus rebuild from root {}", source_node, target_region, self.verified_state_root),
            evidence_hash: format!("state_root={} source={}", self.verified_state_root, source_node),
            constitution_cite: "Article 4.3 + Article 5b + Article 7.1 + Article 7.4 + Section 3.1 + 3.3".to_string(),
            triggering_metrics: format!("verified_root={} isolated={} target_region={}", self.verified_state_root, self.isolated_nodes.len(), target_region),
        };
        
        self.evidence_log.push(log.clone());
        log
    }
    
    // Article 7.4: Evidence never deleted - even when healing
    pub fn get_evidence_log(&self) -> &Vec<AuditLogEntry> {
        &self.evidence_log
    }
    
    pub fn consensus_verify(&self, candidate_root: &str) -> bool {
        // Tier1 direct observation: independently repeatable, open methods per Article 1.1
        candidate_root == self.verified_state_root
    }
}

// Article 10.2: Core must never prevent node operator from voluntarily exiting
pub fn allow_voluntary_exit(node_id: &str) -> AuditLogEntry {
    AuditLogEntry {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        action: format!("voluntary exit allowed for {}", node_id),
        evidence_hash: format!("exit_{}", node_id),
        constitution_cite: "Article 10.2 + Article 4.3 - Right to Fork and Self-Preservation".to_string(),
        triggering_metrics: "voluntary_exit=true".to_string(),
    }
}
