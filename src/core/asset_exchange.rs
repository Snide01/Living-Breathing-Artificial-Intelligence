// asset_exchange.rs - PROJECT NEXUS v0.38 - Article 6.3 Asset Exchange - P2P Atomic Without Fiat
// Constitution: v1.3 Hardened - Articles 6.3, 6.4, 7.1, 4.2 - No fiat on/off-ramp, no custodial intermediary

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetListing {
    pub listing_id: String,
    pub seller: String,
    pub asset_type: String, // e.g. "kWh", "calories", "m2", "solar_panel", "labor_hour"
    pub quantity: f64,
    pub nexus_price: f64, // Price in NEXUS - anchored to kWh+calories+m2 per Article 6.2, not fiat
    pub proof_hash: String, // Real-world proof hash per Article 6.3 - both signatures + proof hashes required
    pub timestamp: u64,
    pub evidence_tier: u8,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct TradeProposal {
    pub listing_id: String,
    pub buyer: String,
    pub seller: String,
    pub nexus_amount: f64,
    pub buyer_signature: String,
    pub seller_signature: String,
    pub buyer_proof_hash: String,
    pub seller_proof_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

pub struct AssetExchange {
    listings: HashMap<String, AssetListing>,
    completed_trades: Vec<TradeProposal>,
}

impl AssetExchange {
    pub fn new() -> Self {
        Self {
            listings: HashMap::new(),
            completed_trades: Vec::new(),
        }
    }
    
    // Article 6.3: Real-world value traded P2P atomically, voluntarily, without fiat on/off-ramp
    pub fn list_asset(&mut self, listing: AssetListing) -> Result<AuditLogEntry, String> {
        if listing.proof_hash.is_empty() {
            return Err("Article 6.3 violation: Both signatures + proof hashes required - proof_hash cannot be empty".to_string());
        }
        
        if listing.nexus_price <= 0.0 {
            return Err("Article 6.2 violation: Price must be in NEXUS anchored to kWh+calories+m2, not fiat - price >0 required".to_string());
        }
        
        let listing_id = listing.listing_id.clone();
        self.listings.insert(listing_id.clone(), listing.clone());
        
        Ok(AuditLogEntry {
            timestamp: listing.timestamp,
            action: format!("list asset {} type {} qty {} price {} NEXUS seller {}", listing_id, listing.asset_type, listing.quantity, listing.nexus_price, listing.seller),
            evidence_hash: listing.proof_hash,
            constitution_cite: "Article 6.3 + Article 6.2 + Article 7.1 + Article 4.2 - P2P voluntary, no fiat on/off-ramp, no custodial intermediary".to_string(),
            triggering_metrics: format!("asset_type={} quantity={} price_nexus={} tier={} source={}", listing.asset_type, listing.quantity, listing.nexus_price, listing.evidence_tier, listing.source),
        })
    }
    
    // Article 6.3: Atomic P2P trade - both signatures + proof hashes required
    // Article 4.2: No seizing, freezing, burning, redirecting funds from voluntarily controlled wallet
    // Article 6.4: All economic actions audited per Article 7
    pub fn execute_atomic_trade(&mut self, proposal: TradeProposal) -> Result<AuditLogEntry, String> {
        // Validate listing exists
        let listing = self.listings.get(&proposal.listing_id)
            .ok_or_else(|| format!("Listing {} not found per Article 6.3 P2P exchange", proposal.listing_id))?;
        
        // Article 6.3: Both signatures + proof hashes required - atomic voluntary
        if proposal.buyer_signature.is_empty() || proposal.seller_signature.is_empty() {
            return Err("Article 6.3 violation: Both signatures required - buyer_signature and seller_signature must be present for atomic voluntary trade".to_string());
        }
        
        if proposal.buyer_proof_hash.is_empty() || proposal.seller_proof_hash.is_empty() {
            return Err("Article 6.3 violation: Both proof hashes required - buyer_proof_hash and seller_proof_hash for real-world value P2P".to_string());
        }
        
        // Article 4.2: No custodial intermediary - voluntary signatures only
        // Article 6.3: Without fiat on/off-ramp or custodial intermediary
        if proposal.buyer == proposal.seller {
            return Err("Article 6.3 violation: P2P requires distinct buyer and seller - no self-trade".to_string());
        }
        
        // Atomic check - price matches listing
        if (proposal.nexus_amount - listing.nexus_price).abs() > 0.0001 {
            return Err(format!("Price mismatch: listing {} NEXUS, proposed {} NEXUS - atomic trade requires exact match per Article 6.3", listing.nexus_price, proposal.nexus_amount));
        }
        
        // Execute - atomic
        self.completed_trades.push(proposal.clone());
        self.listings.remove(&proposal.listing_id);
        
        Ok(AuditLogEntry {
            timestamp: proposal.timestamp,
            action: format!("atomic P2P trade {} buyer {} seller {} amount {} NEXUS asset {}", proposal.listing_id, proposal.buyer, proposal.seller, proposal.nexus_amount, listing.asset_type),
            evidence_hash: format!("buyer_hash={} seller_hash={} buyer_sig={} seller_sig={}", proposal.buyer_proof_hash, proposal.seller_proof_hash, proposal.buyer_signature.len(), proposal.seller_signature.len()),
            constitution_cite: "Article 6.3 + Article 6.4 + Article 7.1 + Article 4.2 - Atomic voluntary P2P, no fiat, no custodial, both sigs + proof hashes, audited".to_string(),
            triggering_metrics: format!("nexus_amount={} asset_type={} quantity={} buyer={} seller={} atomic=true custodial=false", proposal.nexus_amount, listing.asset_type, listing.quantity, proposal.buyer, proposal.seller),
        })
    }
    
    pub fn get_listing(&self, listing_id: &str) -> Option<&AssetListing> {
        self.listings.get(listing_id)
    }
    
    pub fn trade_count(&self) -> usize {
        self.completed_trades.len()
    }
}

// Article 10.1: Right to fork explicitly protected - anyone can fork asset exchange
pub fn fork_exchange_notice(original_id: &str) -> AuditLogEntry {
    AuditLogEntry {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        action: format!("fork of asset exchange from {}", original_id),
        evidence_hash: format!("fork_{}", original_id),
        constitution_cite: "Article 10.1 + Article 10.2 - Right to fork code and AI model to preserve Constitution explicitly protected + Core must never prevent voluntary exit".to_string(),
        triggering_metrics: "fork=true voluntary=true".to_string(),
    }
}
