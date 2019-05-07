use std::net::{ UdpSocket, SocketAddr };
use std::{ str, thread };

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address)?;
    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        debug!("Handling data from {}", src);
        let socket = server_socket.try_clone()?;
        thread::spawn(move || {
            handler(socket, src, &buf[..size]).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

fn handler(socket: UdpSocket, dest: SocketAddr, buf: &[u8]) -> Result<(), failure::Error> {
    print!("{}", str::from_utf8(buf)?);
    socket.send_to(&buf, dest)?;
    Ok(())
}
