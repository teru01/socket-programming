use std::net::TcpStream;
use std::{str, env};
use std::time::Duration;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify address and port. Usage: ./main [address]:[port]");
        std::process::exit(1);
    }
    let remote = args[1].parse().expect("Usage: ./main [address]:[port]");

    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect.");
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(2))).unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).expect("failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("failed to read from the socket");
        print!("{}", str::from_utf8(&buffer).expect("failed to convert to a String"));
    }
}
