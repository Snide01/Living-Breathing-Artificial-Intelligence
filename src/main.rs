use nexus_core::core::genesis::GenesisBlock;

fn main() {
    let genesis = GenesisBlock::genesis();
    println!("=== PROJECT NEXUS GENESIS ===");
    println!("GENESIS REAL HASH: {} *CONSTITUTION.md", genesis.constitution_hash);
    println!("Constitution: {} - Article 4 locked: {}", genesis.constitution_version, genesis.article_4_locked);
    println!("Timestamp: {} - Version: {}", genesis.timestamp, genesis.version);
    println!("Previous: {} - Merkle: {}", genesis.previous_hash, genesis.merkle_root);
    println!("Enactment: {}", genesis.enactment_clause);
    let log = genesis.genesis_audit_log();
    println!("\nArticle 7.1 Audit Log:");
    println!(" {} - {} - cite: {} - hash: {}", log.timestamp, log.action, log.constitution_cite, log.evidence_hash);
    println!("\n9 Core files: 2ddbdaf replication + 362fd05 governance + HeartbeatPool + fb53aab healing NEW + 15% ceiling + cartel-killer 75.0 + kWh+calories+m2 + atomic P2P + 9c6b4b1 genesis");
    println!("Validator set: {:?}", genesis.validator_set);
    println!("Warehouses die, mycelium lives!");
}