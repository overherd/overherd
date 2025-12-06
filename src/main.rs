pub mod net;

#[tokio::main]
async fn main() {
    tokio::spawn(async { net::local_server().await });
    tokio::spawn(async { net::remote_server().await });
    loop {}
}
