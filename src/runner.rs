use cli::Data;
use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::Error;

#[derive(Deserialize)]
#[derive(Debug)]
struct Config {
    token: String,
    customer_id: String,
}

pub fn run(data: &Data) {
    match load_config(data) {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Severo error {:?}", e),
    };
}

fn load_config(data: &Data) -> Result<Config, Error> {
    let config_file = match data.config {
        Some(ref config_file) => config_file.clone(),
        None => String::from("config.json"),
    };

    let config = load_config_from_file(&config_file)?;
    Ok(config)
}

fn load_config_from_file(file_path: &String) -> Result<Config, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("File not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let c: Config = serde_json::from_str(buf.as_str())?;
    Ok(c)
}

fn run_with_config(config: &Config) {}
