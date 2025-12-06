use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn process(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        socket.write_all(b"HELLO THERE\n").await.expect("");
        let n = socket.read(&mut buffer).await.expect("failed t");
        if n == 0 {
            return;
        }
        socket.write_all(&buffer[0..n]).await.expect("");
    }
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
