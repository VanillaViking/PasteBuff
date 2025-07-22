use std::sync::{Arc, Mutex};

use log::{debug, error, info};
use server::{Handler, Request};
use store::KVStore;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

mod config;
mod server;
mod store;
use config::parse_args;

#[tokio::main]
async fn main() {
    let config = parse_args();
    colog::basic_builder().filter(None, config.log_level).init();

    let store = Arc::new(Mutex::new(KVStore::new(4)));
    {
        store
            .lock()
            .unwrap()
            .insert("adsf".to_owned(), "be".to_owned());
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .expect("should be able to bind");

    info!("Server listening on port {}", config.port);
    loop {
        if let Ok((socket, _)) = listener.accept().await {
            let _ = socket
                .peer_addr()
                .inspect(|addr| info!("Connection established to {}", addr));

            let mut handler = Handler::new(socket, store.clone());
            tokio::spawn(async move {
                handler.handle_stream().await;
            });
        } else {
            error!("Could not accept connection");
        }
    }
}
