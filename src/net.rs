use tokio::net::TcpListener;

use crate::settings::Settings;
use std::io;

pub mod comm;
pub mod local;
pub mod remote;

pub async fn local_server() -> io::Result<()> {
    let settings = Settings::new().expect("message");
    let local_port = settings.ports.local;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port))
        .await
        .expect(&format!("Failed binding to {}", local_port));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { local::process(socket).await });
    }
}

pub async fn remote_server() -> io::Result<()> {
    let settings = Settings::new().expect("message");
    let remote_port = settings.ports.remote;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", remote_port))
        .await
        .expect(&format!("Failed binding to port {}", remote_port));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { remote::process(socket).await });
    }
}
