use std::net::UdpSocket;
use std::thread;
use std::str;

fn main() {
    let server_socket = UdpSocket::bind("127.0.0.1:12345")
                           .expect("Could not bind socket");
    loop {
        let mut buf = [0u8; 3000];
        let client_socket = server_socket.try_clone().expect("failed to clone socket");

        match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                // server_socket.send_to(&buf, src).expect("fail");
                thread::spawn(move || {
                    println!("handling data from {}", src);
                    println!("{} bytes received", size.to_string());
                    // server_socket.send_to(&buf, src).expect("fail");
                    print!("{}", str::from_utf8(&buf[..size]).expect("failed to read"));
                    client_socket.send_to(&buf[..size], src).expect("failed to send response");
                });
            },
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }
}

