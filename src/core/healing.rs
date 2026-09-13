// healing.rs - PROJECT NEXUS v0.38 - Full Blackout Gear + Article 7.4
// Constitution: v1.4 TRUE 45a12ef3 - Articles 3.1, 3.3, 5b, 7.1, 7.4, 10.2, 10.6.4
// Fixes: Full mesh blackout -> deterministic convergence without quorum
// + Boot sequence gate -> prevents stale poisoning

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

// ========== FULL BLACKOUT GEAR - Deterministic Tiebreaker ==========
// Article 3.3: Consensus-verified rebuilding must converge without live quorum

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeedCandidate {
    pub height: u64, // epoch merkle root height - highest wins
    pub root_hash: String, // lexicographic tiebreak 2
    pub core_number: u8, // lowest Core number wins - pre-agreed
    pub node_id: String,
}

impl SeedCandidate {
    pub fn from_local_state(node_id: String, core_number: u8, verified_root: String, height: u64) -> Self {
        Self {
            height,
            root_hash: verified_root,
            core_number,
            node_id,
        }
    }
}

/// Deterministic rule: "node with most recent epoch Merkle root becomes seed,
/// tiebreak by lowest Core number, then highest root hash"
/// All 5 nodes run same function -> same seed without quorum
pub fn deterministic_seed(candidates: Vec<SeedCandidate>) -> Option<SeedCandidate> {
    if candidates.is_empty() { return None; }
    // Sort: max height -> min core_number -> max root_hash
    let mut sorted = candidates;
    sorted.sort_by(|a, b| {
        b.height.cmp(&a.height) // highest height first
           .then(a.core_number.cmp(&b.core_number)) // lowest core first
           .then(b.root_hash.cmp(&a.root_hash)) // highest hash final tiebreak
    });
    Some(sorted[0].clone())
}

// ========== BOOT SEQUENCE GATE - Prevents Stale Poisoning ==========

#[derive(Debug, Clone)]
pub enum BootDecision {
    SyncAndJoin { target: SeedCandidate, local_height: u64 },
    StartAsAggregator { reason: String },
    WaitForSeed { reason: String },
    AlreadySynced,
}

pub struct BootGate {
    pub local_candidate: SeedCandidate,
}

impl BootGate {
    pub fn new(local: SeedCandidate) -> Self {
        Self { local_candidate: local }
    }

    /// Check peers BEFORE accepting new work - Article 3.3
    pub fn decide_boot(&self, peers: Vec<SeedCandidate>) -> (BootDecision, AuditLogEntry) {
        if peers.is_empty() {
            // Full blackout solo boot - use tiebreaker on self
            let decision = BootDecision::StartAsAggregator {
                reason: format!("no peers, tiebreaker self core {} height {}",
                    self.local_candidate.core_number, self.local_candidate.height)
            };
            let log = AuditLogEntry {
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                action: format!("boot gate: solo seed {}", self.local_candidate.node_id),
                evidence_hash: format!("height={} core={} root={}",
                    self.local_candidate.height, self.local_candidate.core_number, self.local_candidate.root_hash),
                constitution_cite: "Article 3.3 + 7.1 - Full blackout deterministic seed".to_string(),
                triggering_metrics: "peers=0 boot_gate=solo".to_string(),
            };
            return (decision, log);
        }

        // Find highest peer
        let mut all = peers.clone();
        all.push(self.local_candidate.clone());
        let seed = deterministic_seed(all).unwrap();

        if seed.node_id == self.local_candidate.node_id {
            // I am seed by tiebreaker - but check if I need to wait for higher peer that hasn't booted yet
            // If my height is highest, I start
            let highest_peer_height = peers.iter().map(|p| p.height).max().unwrap_or(0);
            if self.local_candidate.height >= highest_peer_height {
                let decision = BootDecision::StartAsAggregator {
                    reason: format!("tiebreaker won: core {} height {} >= peer max {}",
                        self.local_candidate.core_number, self.local_candidate.height, highest_peer_height)
                };
                let log = AuditLogEntry {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    action: format!("boot gate: seed is self {}", self.local_candidate.node_id),
                    evidence_hash: format!("seed={} height={} peers={}", seed.node_id, seed.height, peers.len()),
                    constitution_cite: "Article 3.3 - Deterministic seed convergence".to_string(),
                    triggering_metrics: format!("local_height={} peer_max={} core={}",
                        self.local_candidate.height, highest_peer_height, self.local_candidate.core_number),
                };
                (decision, log)
            } else {
                // Should not happen due to sort, but safety: sync if peer higher
                let decision = BootDecision::SyncAndJoin {
                    target: seed.clone(),
                    local_height: self.local_candidate.height
                };
                let log = AuditLogEntry {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    action: format!("boot gate: stale detected local {} < peer {} sync required",
                        self.local_candidate.height, seed.height),
                    evidence_hash: format!("local={} peer={} seed={}",
                        self.local_candidate.height, seed.height, seed.node_id),
                    constitution_cite: "Article 3.3 + 3.1 - Prevent stale poisoning".to_string(),
                    triggering_metrics: format!("stale=true local={} peer={}", self.local_candidate.height, seed.height),
                };
                (decision, log)
            }
        } else {
            // Peer is seed - must sync before serving
            if seed.height > self.local_candidate.height {
                let decision = BootDecision::SyncAndJoin {
                    target: seed.clone(),
                    local_height: self.local_candidate.height
                };
                let log = AuditLogEntry {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    action: format!("boot gate: stale -> sync {}->{} before serving",
                        self.local_candidate.node_id, seed.node_id),
                    evidence_hash: format!("sync_required local={} target={} seed_root={}",
                        self.local_candidate.height, seed.height, seed.root_hash),
                    constitution_cite: "Article 3.3 + 3.1 + 7.1 - Boot gate prevents poisoning".to_string(),
                    triggering_metrics: format!("stale=true local_height={} target_height={} seed_core={}",
                        self.local_candidate.height, seed.height, seed.core_number),
                };
                (decision, log)
            } else if seed.height == self.local_candidate.height && seed.root_hash!= self.local_candidate.root_hash {
                // Same height, different root = divergence - trust tiebreaker seed
                let decision = BootDecision::SyncAndJoin {
                    target: seed.clone(),
                    local_height: self.local_candidate.height
                };
                let log = AuditLogEntry {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    action: format!("boot gate: divergence same height diff root -> sync to tiebreaker {}", seed.node_id),
                    evidence_hash: format!("divergence local_root={} seed_root={}",
                        self.local_candidate.root_hash, seed.root_hash),
                    constitution_cite: "Article 3.3 - Divergence resolved by deterministic tiebreaker".to_string(),
                    triggering_metrics: format!("divergence=true height={}", self.local_candidate.height),
                };
                (decision, log)
            } else {
                let decision = BootDecision::AlreadySynced;
                let log = AuditLogEntry {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    action: format!("boot gate: already synced height {}", self.local_candidate.height),
                    evidence_hash: format!("height={}", self.local_candidate.height),
                    constitution_cite: "Article 3.3".to_string(),
                    triggering_metrics: format!("synced height={}", self.local_candidate.height),
                };
                (decision, log)
            }
        }
    }
}

// ========== Original HealingEngine - v0.37 preserved + v0.38 extended ==========

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

    pub fn detect_compromise(&self, metrics: &HealthMetrics) -> Result<bool, AuditLogEntry> {
        let is_compromised = metrics.network_intrusion_score >= INTRUSION_THRESHOLD
            || metrics.cpu_anomaly >= ANOMALY_THRESHOLD
            || metrics.disk_corruption_detected
            || metrics.consensus_divergence >= 0.5;

        if is_compromised {
            let _log = AuditLogEntry {
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
        self.evidence_log.push(log.clone());
        log
    }

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

    pub fn get_evidence_log(&self) -> &Vec<AuditLogEntry> {
        &self.evidence_log
    }

    pub fn consensus_verify(&self, candidate_root: &str) -> bool {
        candidate_root == self.verified_state_root
    }
}

pub fn allow_voluntary_exit(node_id: &str) -> AuditLogEntry {
    AuditLogEntry {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        action: format!("voluntary exit allowed for {}", node_id),
        evidence_hash: format!("exit_{}", node_id),
        constitution_cite: "Article 10.2 + Article 4.3 - Right to Fork and Self-Preservation".to_string(),
        triggering_metrics: "voluntary_exit=true".to_string(),
    }
}
