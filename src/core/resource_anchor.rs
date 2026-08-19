// resource_anchor.rs - PROJECT NEXUS v0.38 - Article 6.2 Economic Integrity - REAL VALUE ANCHOR
// Constitution: v1.3 Hardened - Articles 6.2, 1, 2, 7 - NOT FIAT - per-person kWh, calories, m2
// Token value anchor is per-person consumption of energy, calories, housing - Oracle sources evidence-first

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResourceQuote {
    pub timestamp: u64,
    pub kwh_per_person: f64, // Energy - direct measurement Tier1 per Article 1.1
    pub calories_per_person: f64, // Food
    pub m2_per_person: f64, // Housing
    pub source: String, // Oracle source - must be cited per Article 2.3 + Article 7
    pub evidence_tier: u8, // 1,2,3 per Article 1
    pub evidence_hash: String,
    pub confidence_label: String, // Established Fact, Probable Conclusion, Contested Claim, Speculative Theory per Article 3
    pub funding_affiliation: String, // Must surface where available per Article 2.1, or "Funding/affiliation not disclosed" per 2.2
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub action: String,
    pub evidence_hash: String,
    pub constitution_cite: String,
    pub triggering_metrics: String,
}

pub struct ResourceAnchorOracle {
    quotes: Vec<ResourceQuote>,
    // Evidence-first: Well-evidenced consensus is strongest Tier2 per Article 1.2
}

impl ResourceAnchorOracle {
    pub fn new() -> Self {
        Self { quotes: Vec::new() }
    }
    
    // Article 6.2 + Article 1 + Article 2: Oracle sources evidence-first
    pub fn submit_quote(&mut self, quote: ResourceQuote) -> Result<AuditLogEntry, String> {
        // Article 2.3 + Article 7.1: Resource oracle sources must be cited
        if quote.source.is_empty() {
            return Err("Article 2.3 violation: Resource oracle sources must be cited per Article 7 - source cannot be empty".to_string());
        }
        
        if quote.evidence_hash.is_empty() {
            return Err("Article 7.1 violation: Every Core action must append immutable log with evidence hash".to_string());
        }
        
        // Article 1.3: Must flag "Consensus Without Open Data" if Tier3
        let final_label = if quote.evidence_tier == 3 {
            format!("{} - Consensus Without Open Data per Article 1.3", quote.confidence_label)
        } else {
            quote.confidence_label.clone()
        };
        
        // Article 3: Never present Contested/Speculative as Established Fact
        if (final_label.contains("Contested") || final_label.contains("Speculative")) && final_label.contains("Established Fact") {
            return Err("Article 3 violation: Never present Contested Claim / Speculative Theory as Established Fact".to_string());
        }
        
        // Article 2.2: Absence stated as "Funding/affiliation not disclosed"
        let funding = if quote.funding_affiliation.is_empty() {
            "Funding/affiliation not disclosed per Article 2.2".to_string()
        } else {
            quote.funding_affiliation.clone()
        };
        
        let log = AuditLogEntry {
            timestamp: quote.timestamp,
            action: format!("resource_anchor quote kwh={} cal={} m2={} source={}", quote.kwh_per_person, quote.calories_per_person, quote.m2_per_person, quote.source),
            evidence_hash: quote.evidence_hash.clone(),
            constitution_cite: "Article 6.2 + Article 1 + Article 2 + Article 7.1".to_string(),
            triggering_metrics: format!("tier={} label={} funding={} source={}", quote.evidence_tier, final_label, funding, quote.source),
        };
        
        self.quotes.push(quote);
        Ok(log)
    }
    
    // Article 6.2: Token value anchor calculation - per-person consumption, not fiat
    pub fn calculate_nexus_value(&self) -> Result<(f64, String), String> {
        if self.quotes.is_empty() {
            return Err("No resource quotes available - Article 6.2 requires per-person kWh, calories, m2".to_string());
        }
        
        // Evidence-first: Use only Tier1 + strong Tier2 per Article 1.2
        let valid_quotes: Vec<&ResourceQuote> = self.quotes.iter()
            .filter(|q| q.evidence_tier <= 2)
            .collect();
        
        if valid_quotes.is_empty() {
            return Err("No Tier1/Tier2 quotes - need independently repeatable direct measurement per Article 1.1 or well-replicated analysis per Article 1.2".to_string());
        }
        
        let avg_kwh: f64 = valid_quotes.iter().map(|q| q.kwh_per_person).sum::<f64>() / valid_quotes.len() as f64;
        let avg_cal: f64 = valid_quotes.iter().map(|q| q.calories_per_person).sum::<f64>() / valid_quotes.len() as f64;
        let avg_m2: f64 = valid_quotes.iter().map(|q| q.m2_per_person).sum::<f64>() / valid_quotes.len() as f64;
        
        // Value = weighted combination of real resources, not fiat per Article 6.2
        let nexus_value = avg_kwh * 0.5 + avg_cal * 0.001 + avg_m2 * 0.3;
        
        Ok((nexus_value, format!("Article 6.2 anchor: {} kWh/person + {} cal/person + {} m2/person = {} NEXUS (not fiat) - {} Tier1/Tier2 sources - evidence-first per Article 1", avg_kwh, avg_cal, avg_m2, nexus_value, valid_quotes.len())))
    }
    
    pub fn get_quotes(&self) -> &Vec<ResourceQuote> {
        &self.quotes
    }
}
