use std::net::UdpSocket;
use std::thread;

fn main() {
    let server_socket = UdpSocket::bind("127.0.0.1:12345")
                           .expect("Could not bind socket");
    loop {
        let mut buf = [0u8; 3000];
        // クロージャの中に移動されるからクローンする。
        let client_socket = server_socket.try_clone().expect("failed to clone socket");

        match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                thread::spawn(move || {
                    println!("handling data from {}", src);
                    println!("{} bytes received", size.to_string());
                    // client_socket.send_to(&buf, src).expect("failed to send response");
                });
            },
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }
}

