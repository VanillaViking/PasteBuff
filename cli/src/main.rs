use std::{env, fs::{self, File, OpenOptions}, io::Write};

use clap::{Parser, Subcommand};
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

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Get { key } => todo!(),
        Commands::Set { key } => todo!(),
        Commands::Size => todo!(),
        Commands::Server { address, port } => {
            let addr = format!("{}:{}", address, port);
            let mut store_path = env::home_dir().expect("No home dir");
            store_path.push(".local/share/pbcli/");
            fs::create_dir_all(&store_path).expect("Could not create storage dir");
            store_path.push("server");
            let mut storage = OpenOptions::new().write(true).create(true).open(store_path).expect("Could not create storage file");
            storage.write_all(addr.as_bytes()).expect("Could not write to storage");
        },
    }
}
