use std::env;
use std::net::{TcpStream, SocketAddr};
use std::io::{Result};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(2).collect();

    if args.len() < 3 {
        panic!("Not enough arguments");
    }
    let ip = &args[0];
    let start_port: u16 = args[1].parse().expect("Invalid start port");
    let end_port: u16 = args[2].parse().expect("Invalid end port");

    let (tx, rx) = mpsc::channel();

    let mut handles: Vec<_> = Vec::new();
    for port in start_port..end_port {
        let tx = tx.clone();
        let ip = ip.clone();

        let handle = thread::spawn(move || {
            let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
            if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
                tx.send(port).unwrap();
            }
        });

        handles.push(handle);
    }

    let mut open_ports = Vec::new();
    for port in rx {
        open_ports.push(port);
    }

    println!("Opened ports {:?}", open_ports );

    Ok (())
}
