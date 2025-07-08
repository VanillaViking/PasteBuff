use std::{
    env::{self},
    path::PathBuf,
    process::exit,
};

use log::LevelFilter;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub log_level: LevelFilter,
    pub store_size: u32,
    pub db_file: PathBuf,
}

impl Config {
    pub fn default() -> Self {
        Self {
            port: 7884,
            log_level: LevelFilter::Info,
            store_size: 200,
            db_file: PathBuf::from("pastebuff.db"),
        }
    }
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();
    let mut n = 0;
    while n < args.len() - 1 {
        match args[n].as_str() {
            "-p" | "--port" => {
                if let Ok(p) = args[n + 1].parse::<u16>() {
                    config.port = p;
                    n += 1;
                } else {
                    eprintln!("Invalid port argument");
                    exit(1);
                }
            }
            "-l" | "--log-level" => {
                let log_level = match args[n + 1].as_str() {
                    "trace" => LevelFilter::Trace,
                    "debug" => LevelFilter::Debug,
                    "info" => LevelFilter::Info,
                    "warn" => LevelFilter::Warn,
                    "error" => LevelFilter::Error,
                    _ => {
                        eprintln!("Invalid log level argument");
                        exit(1);
                    }
                };
                config.log_level = log_level;
                n += 1
            }
            "-s" | "--store-size" => {
                if let Ok(s) = args[n + 1].parse::<u32>() {
                    config.store_size = s;
                    n += 1;
                } else {
                    eprintln!("Invalid store size argument");
                    exit(1);
                }
            }
            "--db" => {
                config.db_file = PathBuf::from(args[n + 1].clone());
            }
            _ => (),
        }
        n += 1
    }

    config
}
