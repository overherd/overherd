use dotenv::dotenv;

pub mod net;
pub mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tokio::spawn(async { net::local_server().await });
    tokio::spawn(async { net::remote_server().await });
    loop {}
}
