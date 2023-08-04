use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    println!("Starting program...");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Up and running! connect to 8080");

    let (tx, _rx) = broadcast::channel(10);

    // let bytes_read = socket.read(&mut buffer).await.unwrap();
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (read_socket, mut writer_socket) = socket.split();
            let mut reader = BufReader::new(read_socket);

            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            println!("END OF THE CHAT");
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer_socket.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }

                }
            }
        });
    }
}
