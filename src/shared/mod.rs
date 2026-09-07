use serde::{Deserialize, Serialize};
pub const QC_INTERVAL_BLOCKS: u64 = 6;
pub const PENDING_MAX: usize = 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PendingEvent {
    Tx { seq: u64, hash: String, payload: Vec<u8> },
    Reclamation { seq: u64, heartbeat_id: String, witness_sig: String },
    AiInference { seq: u64, request_id: u64, partial: Vec<u8> },
}

pub struct PendingBuffer {
    pub events: Vec<PendingEvent>,
    pub frozen: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandoffCert {
    pub from_qc: u64,
    pub pending_snapshot_hash: String,
    pub last_seq: u64,
    pub state_root: String,
    pub aggregator_hash: String,
}

impl PendingBuffer {
    pub fn new() -> Self { Self { events: Vec::new(), frozen: false } }
    pub fn push(&mut self, ev: PendingEvent) -> Result<(), String> {
        if self.frozen { return Err("frozen".into()); }
        if self.events.len() >= PENDING_MAX { return Err("bounded 1024 exceeded".into()); }
        self.events.push(ev); Ok(())
    }
    pub fn freeze_for_handoff(self, from_qc: u64, state_root: &str, aggregator_hash: &str) -> HandoffCert {
        let last_seq = self.events.last().map(|e| match e {
            PendingEvent::Tx { seq, .. } => *seq,
            PendingEvent::Reclamation { seq, .. } => *seq,
            PendingEvent::AiInference { seq, .. } => *seq,
        }).unwrap_or(0);
        HandoffCert {
            from_qc,
            pending_snapshot_hash: format!("snap_{}_{}", from_qc, self.events.len()),
            last_seq,
            state_root: state_root.to_string(),
            aggregator_hash: aggregator_hash.to_string(),
        }
    }
}
