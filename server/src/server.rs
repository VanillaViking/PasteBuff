use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::store::KVStore;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Get(String),
    Set { key: String, val: String },
    Size,
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    GetResponse(String),
    SetResponse(String),
    SizeResponse(u32),
    Error(String),
}

pub struct Handler {
    store: Arc<Mutex<KVStore<String, String>>>,
    reader: BufReader<TcpStream>,
    client_address: String,
}

impl Handler {
    pub fn new(stream: TcpStream, store: Arc<Mutex<KVStore<String, String>>>) -> Self {
        let mut client_address = "??".to_string();
        if let Ok(addr) = stream.peer_addr() {
            client_address = addr.to_string();
        }

        Self {
            store,
            reader: BufReader::new(stream),
            client_address,
        }
    }

    pub async fn handle_stream(&mut self) {
        loop {
            let request = self.read_request().await;
            dbg!(&request);

            if request.is_none() {
                continue;
            }


            let response = match request.expect("checked") {
                Request::Get(get_str) => {
                    let mut store = self.store.lock().expect("Could not acquire store lock");
                    if let Some(val) = store.get(&get_str) {
                        Response::GetResponse(val.to_owned())
                    } else {
                        Response::Error(format!("Could not find key {} in store", get_str)) 
                    }
                },
                Request::Set { key, val } => todo!(),
                Request::Size => todo!(),
                Request::Stop => break,
            };

            self.send_response(response);
        }
    }

    async fn read_request(&mut self) -> Option<Request> {
        if self.reader.fill_buf().await.ok()?.is_empty() {
            return Some(Request::Stop);
        }
        let msg_len = match self.reader.read_u32().await {
            Ok(n) => n,
            Err(e) => {
                error!("Error while reading request: {}", e.to_string());
                return None;
            }
        };

        let mut req_buf = vec![0; msg_len as usize];
        match self.reader.read_exact(&mut req_buf).await {
            Ok(0) => {
                if req_buf.is_empty() {
                    // since buffer is empty, it means client has closed the connection
                    info!("Socket closed by client {}", self.client_address);
                    None
                } else {
                    error!("Connection reset by peer");
                    None
                }
            }
            Err(e) => {
                error!("Could not read from stream: {}", e);
                None
            }
            Ok(n) => {
                debug!("Read {} bytes from client request", n);
                if let Ok(req) =
                    serde_json::from_str::<Request>(&String::from_utf8(req_buf).unwrap())
                {
                    Some(req)
                } else {
                    error!("Failed to parse request");
                    None
                }
            }
        }
    }

    fn send_response(&mut self, response: Response) {
        let msg_str = serde_json::to_string(&response).expect("Failed to parse response object");
        let msg_len = msg_str.len();
        self.reader.write_u32(msg_len as u32);
        self.reader.write_all(msg_str.as_bytes());
    }
}
