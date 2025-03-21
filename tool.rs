use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind};
use std::time::Duration;

fn scan_port(ip: &str, port: u16) -> bool {
    let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
    match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
        Ok(_) => true,
        Err(err) => {
            if err.kind() == ErrorKind::TimedOut {
                false
            } else {
                eprintln!("Error scanning port {}: {}", port, err);
                false
            }
        }
    }
}

fn main() {
    let ip = "127.0.0.1"; // replace with the IP address you want to scan
    let start_port = 1;
    let end_port = 65535;

    println!("Scanning {} for open ports...", ip);

    for port in start_port..=end_port {
        if scan_port(ip, port) {
            println!("Port {} is open", port);
        }
    }
}
