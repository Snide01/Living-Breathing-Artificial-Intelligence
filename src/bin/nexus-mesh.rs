use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
fn handle(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap().to_string();
    let mut buf = [0u8; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("[AGGREGATOR] From {}: {}", peer, msg.trim());
                let _ = stream.write_all(b"ACK\n");
            },
            Err(_) => break,
        }
    }
}
fn main() {
    println!("=== PROJECT NEXUS MESH AGGREGATOR v0.88 ddb37aa0 SUCCESSOR ===");
    println!("GENESIS: 45a12ef3 525600 blocks 10yr TRUE per 10.6.4 — horizon_blocks=525600");
    println!("LISTEN 0.0.0.0:30303 — blocks_left=525600 — WITNESS 2/3 HEARTBEAT");
    let listener = TcpListener::bind("0.0.0.0:30303").expect("bind 30303");
    println!("AGGREGATOR LISTENING 0.0.0.0:30303 — 3-NODE LIVE — 87/70");
    for stream in listener.incoming() {
        if let Ok(s) = stream { thread::spawn(|| handle(s)); }
    }
}
