use tokio::net::TcpListener;

use std::io;
use crate::config;

pub mod comm;
pub mod local;
pub mod remote;

pub async fn local_server() -> io::Result<()> {
    let local_port = config::local_port();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port))
        .await
        .expect(&format!("Failed binding to {}", local_port));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { local::process(socket).await });
    }
}

pub async fn remote_server() -> io::Result<()> {
    let remote_port = config::remote_port();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", remote_port))
        .await
        .expect(&format!("Failed binding to port {}", remote_port));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { remote::process(socket).await });
    }
}
