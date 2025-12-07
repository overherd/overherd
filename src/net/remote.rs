use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const DATA_SIZE_LIMIT: usize = 5_000_000;

pub async fn process(mut socket: TcpStream) {
    let mut buffer = [0; 4];
    loop {
        let n = socket
            .read(&mut buffer)
            .await
            .expect("failed to read from socket");

        if n != 4 {
            println!(
                "couldn't parse command: {} {buffer:?}",
                String::from_utf8_lossy(&buffer)
            );
            return;
        }

        match &buffer {
            b"PUBL" => publish_parse(&mut socket).await,
            _ => {
                println!("unrecognized command: {}", String::from_utf8_lossy(&buffer));
                ()
            }
        }
        buffer = [0; 4];
    }
}

async fn publish_parse(socket: &mut TcpStream) {
    println!(" > PUBLISH COMMAND");
    let mut data_size_buffer = [0; 10];
    let n = socket
        .read(&mut data_size_buffer)
        .await
        .expect("failed to read from socket");
    if n != 10 {
        println!("coudln't read lenght");
        return;
    }

    let hex_string = String::from_utf8_lossy(&data_size_buffer[1..9]);
    let data_size =
        usize::from_str_radix(hex_string.trim(), 16).expect("failed to parse hex string");

    if data_size > DATA_SIZE_LIMIT {
        println!("data size too large");
        // TODO skip that amount of data and reply with ERR
        return;
    }

    println!(" > PUBLISH COMMAND {data_size}");

    let mut buffer = Vec::new();
    buffer.resize(data_size, 0);

    let n = socket
        .read(&mut buffer)
        .await
        .expect("failed to read from socket");
    if n != data_size {
        println!("data size didn't match?: {n}");
        // TODO send error
        return;
    }

    // TODO do something with the data
    println!("{}", String::from_utf8_lossy(&buffer));

    socket
        .write_all(b"OK\n")
        .await
        .expect("failed to write into socket");
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
