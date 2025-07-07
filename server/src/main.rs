use log::{debug, error, info, trace, warn, LevelFilter};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7884").await.unwrap();
    colog::basic_builder().filter(None, LevelFilter::Debug).init();
    
    info!("Server listening on port 7884");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            handle_request(socket).await;
        });
    }
}

async fn handle_request(socket: TcpStream) {
    let mut buf_reader = BufReader::new(socket);
    let mut req = String::new();
    match buf_reader.read_line(&mut req).await {
        Ok(n) => debug!("Read {} bytes from client request", n),
        Err(e) => error!("Could not read from stream: {}", e),
    }
    debug!("Got request string: {}", req);
}
