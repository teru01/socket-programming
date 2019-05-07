use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

/**
 * 指定のソケットアドレスで接続を待ち受ける
 */
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        // スレッドを立ち上げて接続に対処する。
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
    Ok(())
}

/**
 * クライアントからの入力を待ち受け、受信したら同じものを返却する。
 */
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;
    }
}
