use std::net::TcpStream;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;
use serde_json::json;

fn bone_path(id: &str) -> PathBuf {
    let mut p = PathBuf::from(std::env::var("HOME").unwrap_or("/home/snide".to_string()));
    p.push(".nexus");
    let _ = fs::create_dir_all(&p);
    p.push(format!("witness_{}_state.json", id));
    p
}

fn save_bone(id: &str, agg: &str) {
    let path = bone_path(id);
    let data = json!({
        "witness_id": id,
        "aggregator": agg,
        "tier": "TierX",
        "blocks_left": 525600,
        "ts": chrono::Utc::now().timestamp(),
        "bone_anchor": true
    });
    let _ = fs::write(&path, data.to_string());
    println!("[BONE] Saved {} = {}", path.display(), data);
}

fn load_bone(id: &str) {
    let path = bone_path(id);
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            println!("[BONE] Loaded from disk bones: {}", content);
            return;
        }
    }
    println!("[BONE] No bone yet, fresh boot - will create");
}

fn print_help() {
    println!("nexus-witness - Project Nexus mycelium heartbeat");
    println!();
    println!("USAGE:");
    println!(" nexus-witness [WITNESS_ID] [AGG_ADDR]");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help(); return;
    }
    let id_raw = args.get(1).cloned().unwrap_or("2".to_string());
    if!id_raw.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("ERROR: WITNESS_ID must be numeric, got '{}'", id_raw);
        print_help(); std::process::exit(1);
    }
    let agg = args.get(2).cloned().unwrap_or("192.168.50.241:30303".to_string());

    // BONE FIRST - read who we are from bone before flesh
    load_bone(&id_raw);
    save_bone(&id_raw, &agg);

    println!("Starting WITNESS {} -> {}", id_raw, agg);
    loop {
        if let Ok(mut s) = TcpStream::connect(&agg) {
            let ts = chrono::Utc::now().timestamp();
            let msg = format!("WITNESS {} HEARTBEAT {} blocks_left=525600\n", id_raw, ts);
            let _ = s.write_all(msg.as_bytes());
            println!("sent {} to {}", msg.trim(), agg);
        } else {
            eprintln!("AGG {} unreachable, retry in 5s", agg);
        }
        thread::sleep(Duration::from_secs(5));
    }
}
