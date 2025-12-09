use tokio::net::TcpListener;

use std::io;

pub mod comm;
pub mod protocol;
pub mod local;
pub mod remote;

pub async fn local_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999")
        .await
        .expect("Failed binding to port 8080");
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { local::process(socket).await });
    }
}

pub async fn remote_server() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed binding to port 8080");
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { remote::process(socket).await });
    }
}
