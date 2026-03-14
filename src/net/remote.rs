// vim: fdm=indent fdn=1

use super::{comm, protocol};
use crate::net::list::get_peer_list;
use std::{collections::HashSet, fs};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const DATA_SIZE_LIMIT: usize = 5_000_000;
const COMMAND_SIZE: usize = 4;
const COMMAND_DATA_SIZE: usize = 8;

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
            println!(
                "couldn't parse command: {} {n} {buffer:?}",
                String::from_utf8_lossy(&buffer)
            );
            socket.write_all(protocol::NO_REPLY).await.ok();
            return;
        }

        let result = match &buffer {
            protocol::BROADCAST_CMD => receive_broadcast(&mut socket).await,
            protocol::REQ_PEERS_CMD => receive_request_peers(&mut socket).await,
            _ => {
                println!("unrecognized command: {}", String::from_utf8_lossy(&buffer));
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
        println!("coudln't read lenght");
        return Err(());
    }

    let hex_string = String::from_utf8_lossy(&data_size_buffer[1..COMMAND_DATA_SIZE + 1]);
    let data_size =
        usize::from_str_radix(hex_string.trim(), 16).expect("failed to parse hex string");

    if data_size > DATA_SIZE_LIMIT {
        println!("data size too large");
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
        println!("data size didn't match?: {n}");
        // TODO send error
        return Err(());
    }

    return Ok(buffer);
}

// =============================================
//             RECEIVE FUNCTIONS

async fn receive_broadcast(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > BRODCAST COMMAND");

    let data = get_data(socket).await?;

    println!("{}", String::from_utf8_lossy(&data));
    fs::write("dummy", data).expect("fuck i failed");

    let _ = socket.write_all(protocol::OK_REPLY).await;
    Ok(())
}

pub async fn receive_request_peers(socket: &mut TcpStream) -> Result<(), ()> {
    let peers = get_peer_list().await?;
    let data = peers
        .into_iter()
        .collect::<Vec<String>>()
        .join(":")
        .into_bytes();

    let mut message = Vec::new();
    message.extend_from_slice(protocol::RES_PEERS_CMD);
    message.extend_from_slice(b" ");
    let hex_len = format!("{:0>1$x}", data.len(), COMMAND_DATA_SIZE);
    message.extend_from_slice(hex_len.as_bytes());
    message.extend_from_slice(b" ");
    message.extend_from_slice(&data);

    let _ = socket.write_all(&message).await;
    Ok(())
}

// =============================================
//             SEND FUNCTIONS

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

    let peers = get_peer_list().await.unwrap_or(HashSet::new());
    for p in peers {
        if let Err(e) = comm::send_message(format!("{}:8080", p).as_str(), &message).await {
            eprintln!("Connection failed: {}", e);
        }
    }
    Ok(())
}

pub async fn send_request_peers(peer: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut message = Vec::new();
    message.extend_from_slice(protocol::REQ_PEERS_CMD);

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
        println!(
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
