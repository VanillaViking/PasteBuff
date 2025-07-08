use log::{debug, error, info, trace, warn, LevelFilter};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::{TcpListener, TcpStream}};

mod config;
use config::parse_args;

#[tokio::main]
async fn main() {
    let config = parse_args();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).await.unwrap();
    colog::basic_builder().filter(None, config.log_level).init();
    
    info!("Server listening on port {}", config.port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let _ =socket.peer_addr().inspect(|addr| info!("Connection established to {}", addr));
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            handle_request(socket).await;
        });
    }
}

async fn handle_request(socket: TcpStream) {
    let mut buf_reader = BufReader::new(socket);
    loop {
        let mut req = String::new();
        match buf_reader.read_line(&mut req).await {
            Ok(0) => {
                if req.is_empty() {
                    // since buffer is empty, it means client has closed the connection
                    let _ = buf_reader.into_inner().peer_addr().inspect(|addr| info!("Socket closed by client {}", addr));
                    break;
                } else {
                    error!("Connection reset by peer");
                    break;
                }
            },
            Ok(n) => debug!("Read {} bytes from client request", n),
            Err(e) => error!("Could not read from stream: {}", e),
        }
        debug!("Got request string: {}", req);
    }
}
