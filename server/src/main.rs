use std::sync::{Arc, Mutex};

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

    let store = Arc::new(Mutex::new(KVStore::new(4)));
    {
        store.lock().unwrap().insert("adsf".to_owned(), "be".to_owned());
    }
    // store.insert("basdf".to_owned(), 10001);
    // dbg!(&store);
    // store.insert("cda".to_owned(), 10002);
    // dbg!(&store);
    // store.insert("bd".to_owned(), 10003);
    // dbg!(store.get("cda"));
    // dbg!(&store);
    // store.insert("adf".to_owned(), 10004);
    // dbg!(&store);
    // store.insert("baa".to_owned(), 10005);
    // dbg!(&store);

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
            
            let store_lock = store.clone();
            tokio::spawn(async move {
                conn::handle_stream(socket, store_lock).await;
            });
        } else {
            error!("Could not accept connection");
        }
    }
}
