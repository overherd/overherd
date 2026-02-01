use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::settings::Settings;
use std::io;

pub mod comm;
pub mod list;
pub mod local;
pub mod protocol;
pub mod remote;

pub async fn local_server() -> io::Result<()> {
    let settings = Settings::new().expect("message");
    let local_port = settings.ports.local;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port))
        .await
        .expect(&format!("Failed binding to {}", local_port));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let io = TokioIo::new(socket);
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(local::process))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
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
