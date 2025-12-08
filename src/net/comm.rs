use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_message(addr: &str, msg: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect(addr).await?;
    socket
        .write_all(msg)
        .await
        .expect("failed to send data to client");
    let mut buffer = [0; 2];
    socket.read(&mut buffer).await.expect("fucked up");
    println!(" > Got reply {}", String::from_utf8_lossy(&buffer));
    Ok(())
}
