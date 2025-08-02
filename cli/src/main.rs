use std::{env, fs::{self, OpenOptions}, io::{Read, Write}, net::TcpStream};

use anyhow::Result;
use anyhow::anyhow;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
/// Command line utility to access the PasteBuff store
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
   Get {
       key: String
   },
   Set {
       key: String
   },
   Size,
   Server {
       #[arg(short, long)]
       address: String,
       #[arg(short, long)]
       port: u16,
   }
}

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

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Get { key } => todo!(),
        Commands::Set { key } => todo!(),
        Commands::Size => {
            let mut stream = connect().expect("Could not connect to server");
            let req = Request::Size;
            
            let msg_str = serde_json::to_string(&req).expect("Could not encode request");
            stream.write_u32::<BigEndian>(msg_str.len() as u32).expect("Could not write to stream");
            stream.write_all(msg_str.as_bytes()).expect("Could not write to stream");
            match handle_response(&mut stream) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e.to_string()),
            }
        },
        Commands::Server { address, port } => {
            let addr = format!("{}:{}", address, port);
            let mut store_path = env::home_dir().expect("No home dir");
            store_path.push(".local/share/pbcli/");
            fs::create_dir_all(&store_path).expect("Could not create storage dir");
            store_path.push("server");
            let mut storage = OpenOptions::new().write(true).truncate(true).open(store_path).expect("Could not create storage file");
            storage.write_all(addr.as_bytes()).expect("Could not write to storage");
        },
    }
}

fn handle_response(stream: &mut TcpStream) -> anyhow::Result<()> {
    let res_len = stream.read_u32::<BigEndian>()?;
    let mut res_buf = vec![0u8; res_len as usize];
    stream.read_exact(&mut res_buf)?;
    let response = serde_json::from_str::<Response>(&String::from_utf8(res_buf)?)?;
    
    dbg!(response);

    Ok(())


}

fn connect() -> anyhow::Result<TcpStream> {
    let mut store_path = env::home_dir().ok_or(anyhow!("no home dir"))?;
    store_path.push(".local/share/pbcli/server");
    
    let addr = fs::read_to_string(store_path)?;

    let stream = TcpStream::connect(&addr)?;
    Ok(stream)
}
