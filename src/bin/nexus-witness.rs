use std::net::TcpStream;
use std::io::Write;
use std::thread;
use std::time::Duration;
fn main() {
    let id = std::env::args().nth(1).unwrap_or("2".to_string());
    let agg = std::env::args().nth(2).unwrap_or("192.168.50.241:30303".to_string());
    loop {
        if let Ok(mut s) = TcpStream::connect(&agg) {
            let ts = chrono::Utc::now().timestamp();
            let msg = format!("WITNESS {} HEARTBEAT {} blocks_left=525600\n", id, ts);
            let _ = s.write_all(msg.as_bytes());
            println!("sent {} to {}", msg.trim(), agg);
        }
        thread::sleep(Duration::from_secs(5));
    }
}
