use std::sync::{Arc, Mutex};

use log::{debug, error, info};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::TcpStream};

use crate::store::KVStore;

pub async fn handle_stream(socket: TcpStream, store: Arc<Mutex<KVStore<String, String>>>) {
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

fn parse_msg() {}
