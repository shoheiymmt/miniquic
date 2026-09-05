use std::net::UdpSocket;

pub fn run() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    socket.send_to(b"hello", "127.0.0.1:4433")?;

    let mut buf = [0u8; 256];

    let (size, src) = socket.recv_from(&mut buf)?;

    println!("client: received {} bytes from {}: {}", size, src, String::from_utf8_lossy(&buf[..size]));

    Ok(())
}
