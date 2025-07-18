use log::{debug, error, info};
use store::KVStore;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

mod config;
mod store;
mod conn;
use config::parse_args;

#[tokio::main]
async fn main() {
    let config = parse_args();
    colog::basic_builder().filter(None, config.log_level).init();

    let mut store = KVStore::new(4);
    store.insert("basdf".to_owned(), 10001);
    dbg!(&store);
    store.insert("cda".to_owned(), 10002);
    dbg!(&store);
    store.insert("bd".to_owned(), 10003);
    dbg!(store.get("cda"));
    dbg!(&store);
    store.insert("adf".to_owned(), 10004);
    dbg!(&store);
    store.insert("baa".to_owned(), 10005);
    dbg!(&store);

    // dbg!(store.array_get(23423));

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .expect("should be able to bind");

    info!("Server listening on port {}", config.port);
    loop {
        if let Ok((socket, _)) = listener.accept().await {
            let _ = socket
                .peer_addr()
                .inspect(|addr| info!("Connection established to {}", addr));

            tokio::spawn(async move {
                handle_request(socket).await;
            });
        } else {
            error!("Could not accept connection");
        }
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
                    let _ = buf_reader
                        .into_inner()
                        .peer_addr()
                        .inspect(|addr| info!("Socket closed by client {}", addr));
                    break;
                } else {
                    error!("Connection reset by peer");
                    break;
                }
            }
            Ok(n) => debug!("Read {} bytes from client request", n),
            Err(e) => error!("Could not read from stream: {}", e),
        }
        debug!("Got request string: {}", req);
    }
}
