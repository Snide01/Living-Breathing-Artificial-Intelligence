use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::path::PathBuf;
use std::fs;

fn bone_save(genesis_hash: &str, blocks: u64) {
    let home = std::env::var("HOME").unwrap_or("/home/snide".to_string());
    let dir = PathBuf::from(format!("{}/.nexus", home));
    let _ = fs::create_dir_all(&dir);
    let p = dir.join("aggregator_state.json");
    let tmp = dir.join("aggregator_state.json.tmp");
    let data = format!(
        r#"{{"genesis":"{}","blocks_left":{},"blocks":{},"bone_anchor":true,"ts":{}}}"#,
        genesis_hash, blocks, blocks, chrono::Utc::now().timestamp()
    );
    if let Ok(mut f) = fs::File::create(&tmp) {
        let _ = f.write_all(data.as_bytes());
        let _ = f.sync_all();
        let _ = fs::rename(&tmp, &p);
        if let Ok(d) = fs::File::open(&dir) { let _ = d.sync_all(); }
    } else {
        let _ = fs::write(&p, &data);
    }
    println!("[BONE AGG] Saved {} = {}", p.display(), data);
}

fn main() {
    let genesis_str = std::fs::read_to_string("genesis.json").unwrap_or("{}".to_string());
    let genesis_hash = serde_json::from_str::<serde_json::Value>(&genesis_str)
        .ok().and_then(|v| v.get("constitution_hash").and_then(|h| h.as_str()).map(|s| s.to_string()))
        .unwrap_or("unknown".to_string());
    let blocks = serde_json::from_str::<serde_json::Value>(&genesis_str)
        .ok().and_then(|v| v.get("horizon_blocks").and_then(|h| h.as_u64()))
        .unwrap_or(525600);
    bone_save(&genesis_hash, blocks);
    println!("=== PROJECT NEXUS GENESIS ===");
    println!("GENESIS REAL HASH: {} *CONSTITUTION.md", genesis_hash);
    println!("Version: v1.4-TRUE-525600-10yr-{} blocks_left={}", &genesis_hash[..genesis_hash.len().min(8)], blocks);
    println!("Warehouses die, mycelium lives!");
    println!("[AGGREGATOR] LISTEN 0.0.0.0:30303 genesis={} blocks={}", genesis_hash, blocks);
    let listener = TcpListener::bind("0.0.0.0:30303").expect("bind 30303");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => { thread::spawn(move || handle(s)); }
            Err(e) => eprintln!("accept err {}", e),
        }
    }
}

fn handle(stream: TcpStream) {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or("unknown".to_string());
    let reader = BufReader::new(stream);
    for line in reader.lines().flatten() {
        println!("[AGGREGATOR] From {}: {}", peer, line);
        if !line.contains("525600") {
            eprintln!("[AGGREGATOR] WARN {} genesis mismatch", peer);
        }
    }
}
