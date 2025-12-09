// vim: fdm=indent fdn=1
use super::{comm, protocol};
use std::fs;
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
            protocol::BROADCAST_CMD => broadcast_parse(&mut socket).await,
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

// =============================================
//             PARSE FUNCTIONS

async fn broadcast_parse(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > BRODCAST COMMAND");
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

    // TODO do something with the data
    println!("{}", String::from_utf8_lossy(&buffer));
    fs::write("dummy", buffer).expect("fuck i failed");

    socket.write_all(protocol::OK_REPLY).await.ok();
    Ok(())
}

// =============================================
//             COMMAND FUNCTIONS

pub async fn broadcast(data: Vec<u8>) -> Result<(), ()> {
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
    if let Err(e) = comm::send_message("overherd-node-2:8080", &message).await {
        eprintln!("Connection failed: {}", e);
    }
    Ok(())
}
