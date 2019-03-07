use std::net::UdpSocket;
use std::{str, io};
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:54321").expect("failed to bind socket");
    socket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();

    let file = File::open("test.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();
    // io::stdin().read_line(&mut input).unwrap();

    socket.send_to(input.as_bytes(), "127.0.0.1:12345").expect("failed to send data");

        // let mut buffer = [0u8; 1500];
        // socket.recv_from(&mut buffer).expect("failed to receive");
        // print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));
}

// /**
//  * UdpSocketを作成してアドレスにバインドすれば、そのアドレスに対して送受信できる
//  * connectでリモートへ接続していれば、システムコールのsend, recvで送受信できる
//  * 
//  * /
