// vim: fdm=indent fdn=1
use super::{protocol, remote::broadcast};
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
            protocol::PUBLISH_CMD => publish_parse(&mut socket).await,
            protocol::NEIGHBOUR_ADD_CMD => add_neightbour_parse(&mut socket).await,
            protocol::NEIGHBOUR_LIST_CMD => list_neightbour_parse(&mut socket).await,
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

async fn list_neightbour_parse(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > LIST NEIGHBOUR COMMAND");
    socket.read_u8().await.unwrap_or(0); // read new line
    list_neighbour(socket).await
}

async fn add_neightbour_parse(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > ADD NEIGHBOUR COMMAND");
    // TODO parse the ip that should be given
    socket.read_u8().await.unwrap_or(0); // read new line
    add_neighbour()
}

async fn publish_parse(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > PUBL COMMAND");
    let mut data_size_buffer = [0; COMMAND_DATA_SIZE + 2];
    let n = socket.read(&mut data_size_buffer).await.unwrap_or(0);

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

    broadcast(buffer).await?;
    Ok(())
}

// =============================================
//             COMMAND FUNCTIONS

async fn list_neighbour(socket: &mut TcpStream) -> Result<(), ()> {
    socket.write_all(b"[]").await.map_err(|_| ())
}

fn add_neighbour() -> Result<(), ()> {
    Ok(())
}
