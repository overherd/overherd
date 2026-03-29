// vim: fdm=indent fdn=1

use crate::settings::Settings;
use super::{comm, protocol};
use crate::net::{
    INSTANCE_ID,
    list::{self, get_peer_list},
};
use std::{collections::HashSet, fs};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::Duration,
    time::timeout,
};

const DATA_SIZE_LIMIT: usize = 5_000_000;
const COMMAND_SIZE: usize = 4;
const COMMAND_DATA_SIZE: usize = 8;
const PROTOCOL_TIMOUT: u64 = 2;

const LEARN_REQUESTING_PEER_RATIO: f64 = 0.2; // 20 %

pub async fn process(mut socket: TcpStream) {
    loop {
        let mut buffer = [0; COMMAND_SIZE];
        let n = socket
            .read(&mut buffer)
            .await
            .expect("failed to read from socket");
        if n == 0 {
            return;
        }

        if n != COMMAND_SIZE {
            eprintln!(
                "couldn't parse command: {} {n} {buffer:?}",
                String::from_utf8_lossy(&buffer)
            );
            socket.write_all(protocol::NO_REPLY).await.ok();
            return;
        }

        let result = match &buffer {
            protocol::REQUEST_ID_CMD => receive_request_id(&mut socket).await,
            protocol::BROADCAST_CMD => receive_broadcast(&mut socket).await,
            protocol::REQUEST_PEERS_CMD => receive_request_peers(&mut socket).await,
            _ => {
                eprintln!("unrecognized command: {}", String::from_utf8_lossy(&buffer));
                Err(())
            }
        };

        match result {
            Ok(_) => {
                socket.write_all(protocol::OK_REPLY).await.ok();
            }
            Err(_) => {
                socket.write_all(protocol::NO_REPLY).await.ok();
                return;
            }
        }
    }
}

async fn get_data(socket: &mut TcpStream) -> Result<Vec<u8>, ()> {
    let mut data_size_buffer = [0; COMMAND_DATA_SIZE + 2];
    let n = socket
        .read(&mut data_size_buffer)
        .await
        .expect("failed to read from socket");
    if n != COMMAND_DATA_SIZE + 2 {
        eprintln!("coudln't read lenght");
        return Err(());
    }

    let hex_string = String::from_utf8_lossy(&data_size_buffer[1..COMMAND_DATA_SIZE + 1]);
    let data_size =
        usize::from_str_radix(hex_string.trim(), 16).expect("failed to parse hex string");

    if data_size > DATA_SIZE_LIMIT {
        eprintln!("data size too large");
        // TODO skip that amount of data and reply with ERR
        return Err(());
    }

    let mut buffer = Vec::new();
    buffer.resize(data_size, 0);

    let n = socket
        .read(&mut buffer)
        .await
        .expect("failed to read from socket");
    if n != data_size {
        eprintln!("data size didn't match?: {n}");
        // TODO send error
        return Err(());
    }

    return Ok(buffer);
}

/// =============================================
///             RECEIVE FUNCTIONS

/// Protocol function for replying to peer id:
///
/// RSID [size] [node-id]
async fn receive_request_id(socket: &mut TcpStream) -> Result<(), ()> {
    if let Some(instance_id) = INSTANCE_ID.get() {
        let mut message = Vec::new();
        message.extend_from_slice(protocol::RESPONSE_ID_CMD);
        message.extend_from_slice(b" ");
        let hex_len = format!("{:0>1$x}", instance_id.len(), COMMAND_DATA_SIZE);
        message.extend_from_slice(hex_len.as_bytes());
        message.extend_from_slice(b" ");
        message.extend_from_slice(instance_id.as_bytes());

        let _ = socket.write_all(&message).await;
    } else {
        eprintln!("Instance Id uninitialized");
        return Err(());
    }
    Ok(())
}

async fn receive_broadcast(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > BRODCAST");

    let data = get_data(socket).await?;

    println!("{}", String::from_utf8_lossy(&data));
    fs::write("dummy", data).expect("fuck i failed");

    let _ = socket.write_all(protocol::OK_REPLY).await;
    Ok(())
}

pub async fn receive_request_peers(socket: &mut TcpStream) -> Result<(), ()> {
    let settings = Settings::get();
    let peers = get_peer_list()?;
    let data = peers
        .into_iter()
        .collect::<Vec<String>>()
        .join(":")
        .into_bytes();

    let mut message = Vec::new();
    message.extend_from_slice(protocol::RESPONSE_PEERS_CMD);
    message.extend_from_slice(b" ");
    let hex_len = format!("{:0>1$x}", data.len(), COMMAND_DATA_SIZE);
    message.extend_from_slice(hex_len.as_bytes());
    message.extend_from_slice(b" ");
    message.extend_from_slice(&data);

    let r: f64 = rand::random();
    if r < settings.gossip.learn_requesting_peer_ratio {
        let _ = list::append_peer_list(vec![socket.peer_addr().unwrap().ip().to_string()]);
    }

    let _ = socket.write_all(&message).await;
    Ok(())
}

/// =============================================
///             SEND FUNCTIONS

/// Protocol function for requesting peer id:
///
/// RQID
///
/// Returns peer id string
pub async fn send_request_id(peer: &String) -> Result<String, Box<dyn std::error::Error>> {
    async fn _socket(peer: &String) -> Result<String, Box<dyn std::error::Error>> {
        let mut message = Vec::new();
        message.extend_from_slice(protocol::REQUEST_ID_CMD);
        let mut socket = TcpStream::connect(format!("{}:8080", peer)).await?;
        socket
            .write_all(&message)
            .await
            .expect("failed to send data to client");

        // get peer reply
        let mut buffer = [0; COMMAND_SIZE];
        let n = socket
            .read(&mut buffer)
            .await
            .expect("failed to read from socket");
        if n == 0 {
            return Err(format!("").into());
        }

        if n != COMMAND_SIZE {
            eprintln!(
                "couldn't parse command: {} {n} {buffer:?}",
                String::from_utf8_lossy(&buffer)
            );
            socket.write_all(protocol::NO_REPLY).await.ok();
            return Err(format!("").into());
        }

        let data = get_data(&mut socket).await.unwrap_or(Vec::new());
        socket.shutdown().await?;

        Ok(String::from_utf8_lossy(&data).into())
    }

    match timeout(Duration::from_secs(PROTOCOL_TIMOUT), _socket(peer)).await {
        Ok(o) => o,
        Err(_) => Err(format!("timeout").into()),
    }
}

pub async fn send_broadcast(data: Vec<u8>) -> Result<(), ()> {
    let mut message = Vec::new();
    message.extend_from_slice(protocol::BROADCAST_CMD);
    message.extend_from_slice(b" ");
    let hex_len = format!("{:0>1$x}", data.len(), COMMAND_DATA_SIZE);
    message.extend_from_slice(hex_len.as_bytes());
    message.extend_from_slice(b" ");
    message.extend_from_slice(&data);
    println!(
        " > Sending BROADCAST: {}",
        String::from_utf8_lossy(&message)
    );

    let peers = get_peer_list().unwrap_or(HashSet::new());
    for p in peers {
        if let Err(e) = comm::send_message(format!("{}:8080", p).as_str(), &message).await {
            eprintln!("Connection failed: {}", e);
        }
    }
    Ok(())
}

pub async fn send_request_peers(peer: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut message = Vec::new();
    message.extend_from_slice(protocol::REQUEST_PEERS_CMD);

    println!("Requesting peers from: {}", peer);
    let mut socket = TcpStream::connect(format!("{}:8080", peer)).await?;
    socket
        .write_all(&message)
        .await
        .expect("failed to send data to client");

    // get peer reply
    let mut buffer = [0; COMMAND_SIZE];
    let n = socket
        .read(&mut buffer)
        .await
        .expect("failed to read from socket");
    if n == 0 {
        return Err(format!("").into());
    }

    if n != COMMAND_SIZE {
        eprintln!(
            "couldn't parse command: {} {n} {buffer:?}",
            String::from_utf8_lossy(&buffer)
        );
        socket.write_all(protocol::NO_REPLY).await.ok();
        return Err(format!("").into());
    }

    let data = get_data(&mut socket).await.unwrap_or(Vec::new());
    socket.shutdown().await?;
    if data.is_empty() {
        return Ok(Vec::new());
    }
    Ok(String::from_utf8_lossy(&data)
        .split(":")
        .map(str::to_string)
        .collect())
}
