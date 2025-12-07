// vim: fdm=indent fdn=1
use std::fs;

use super::comm;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const DATA_SIZE_LIMIT: usize = 5_000_000;
const COMMAND_SIZE: usize = 4;
const COMMAND_DATA_SIZE: usize = 8;

const BROADCAST_CMD: &[u8; 4] = b"BROD";

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
            socket.write_all(b"NO").await.expect("");
            return;
        }

        let result = match &buffer {
            BROADCAST_CMD => broadcast_parse(&mut socket).await,
            _ => {
                println!("unrecognized command: {}", String::from_utf8_lossy(&buffer));
                Err(())
            }
        };

        // Send Error message
        if let Err(_) = result {
            socket.write_all(b"NO").await.expect("");
            return;
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

    println!(" > BRODCAST COMMAND {data_size}");

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

    socket
        .write_all(b"OK")
        .await
        .expect("failed to write into socket");
    Ok(())
}

// =============================================
//             COMMAND FUNCTIONS

pub async fn broadcast(data: Vec<u8>) {
    let mut message = Vec::new();
    message.extend_from_slice(BROADCAST_CMD);
    message.extend_from_slice(b" ");
    let hex_len = format!("{:08x}", data.len());
    message.extend_from_slice(hex_len.as_bytes());
    message.extend_from_slice(b" ");
    message.extend_from_slice(&data);
    println!(
        " > Sending BROADCAST: {}",
        String::from_utf8_lossy(&message)
    );
    comm::send_message(&message).await;
}
