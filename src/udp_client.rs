use std::net::{UdpSocket, SocketAddr};
use std::{str, io, env};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("please specify target address and port. Usage: ./main [address]:[port]");
        std::process::exit(1);
    }
    let remote: SocketAddr = args[1].parse().expect("Usage: ./main [address]:[port]");

    let socket = UdpSocket::bind("127.0.0.1:0").expect("failed to bind socket");
    socket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        socket.send_to(input.as_bytes(), &remote).expect("failed to send data");

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));
    }
}
