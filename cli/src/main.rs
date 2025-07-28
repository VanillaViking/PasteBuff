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
    dbg!(args);
}
