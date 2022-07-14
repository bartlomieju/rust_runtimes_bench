use std::io;
use async_std::task;
use async_std::io::WriteExt;
use async_std::io::ReadExt;
use async_std::net::TcpListener;
use async_std::net::TcpStream;

async fn process_socket(mut socket: TcpStream) {
    socket.set_nodelay(true).unwrap();
    let mut req = [0; 4096];
    let res = b"HTTP/1.1 200 OK\r\nContent-length: 12\r\n\r\nHello world\n";

    loop {
        let n = socket.read(&mut req).await.unwrap();
        if n == 0 {
            return;
        }
        socket.write_all(res).await.unwrap();
    }
}

async fn main_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        task::spawn(process_socket(socket));
    }
}

fn main() {
    task::block_on(main_server()).unwrap()
}