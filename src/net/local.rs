use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use crate::net::remote::{broadcast};

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
            socket.write_all(b"NO").await.expect("");
            return;
        }

        let result = match &buffer {
            b"PUBL" => broadcast_parse(&mut socket).await,
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

async fn broadcast_parse(socket: &mut TcpStream) -> Result<(), ()> {
    println!(" > BROD COMMAND");
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

    println!(" > BROD COMMAND {data_size}");

    let mut buffer = Vec::new();
    buffer.resize(data_size, 0);

    let n = socket
        .read_exact(&mut buffer)
        .await
        .expect("failed to read from socket");
    if n != data_size {
        println!("data size didn't match?: {n}");
        // TODO send error
        return Err(());
    }

    broadcast(buffer).await;

    socket
        .write_all(b"OK\n")
        .await
        .expect("failed to write into socket");
    Ok(())
}

// async fn client() {
//     let mut socket = TcpStream::connect(OTHER_IP)
//         .await
//         .expect("failed to connect to client");
//     socket
//         .write_all(b"hello world!")
//         .await
//         .expect("failed to send data to client");
//
//     let mut buffer = [0; 1024];
//     socket.read(&mut buffer).await.expect("fucked up");
//     println!("{}", String::from_utf8_lossy(&buffer[..1024]));
// }
