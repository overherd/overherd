use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_message(msg: &[u8]) {
    let mut socket = TcpStream::connect("192.168.8.120:8080")
        .await
        .expect("failed to connect to client");
    socket
        .write_all(msg)
        .await
        .expect("failed to send data to client");
    let mut buffer = [0; 2];
    socket.read(&mut buffer).await.expect("fucked up");
    println!(" > Got reply {}", String::from_utf8_lossy(&buffer));
}
