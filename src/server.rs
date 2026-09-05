use std::net::UdpSocket;

pub fn run() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:4433")?;
    
    let mut buf = [0u8; 256];

    let (size, src) = socket.recv_from(&mut buf)?;

    println!("server: received {} bytes from {}", size, src);

    socket.send_to(&buf[..size], src)?;

    Ok(())
}
