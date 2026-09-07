use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};
use std::thread;

fn main() {
    let genesis_str = std::fs::read_to_string("genesis.json").unwrap_or("{}".to_string());
    let genesis_hash = serde_json::from_str::<serde_json::Value>(&genesis_str)
        .ok().and_then(|v| v.get("constitution_hash").and_then(|h| h.as_str()).map(|s| s.to_string()))
        .unwrap_or("unknown".to_string());
    let blocks = serde_json::from_str::<serde_json::Value>(&genesis_str)
        .ok().and_then(|v| v.get("horizon_blocks").and_then(|h| h.as_u64()))
        .unwrap_or(525600);

    println!("=== PROJECT NEXUS GENESIS ===");
    println!("GENESIS REAL HASH: {} *CONSTITUTION.md", genesis_hash);
    println!("Version: v1.4-TRUE-525600-10yr-{} blocks_left={}", &genesis_hash[0..8], blocks);
    println!("Warehouses die, mycelium lives!");
    println!("[AGGREGATOR] LISTEN 0.0.0.0:30303 genesis={} blocks={}", genesis_hash, blocks);

    let listener = TcpListener::bind("0.0.0.0:30303").expect("bind 30303");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle(s));
            }
            Err(e) => eprintln!("accept err {}", e),
        }
    }
}

fn handle(stream: TcpStream) {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or("unknown".to_string());
    let reader = BufReader::new(stream);
    for line in reader.lines().flatten() {
        println!("[AGGREGATOR] From {}: {}", peer, line);
        // simple validation
        if !line.contains("525600") {
            eprintln!("[AGGREGATOR] WARN {} genesis mismatch blocks_left !=525600", peer);
        }
    }
}
