use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let remote = "127.0.0.1:33333".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect.");
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).expect("failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("failed to read from socket");
        print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));
    }
}
