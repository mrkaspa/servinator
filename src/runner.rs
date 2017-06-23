use cli::Data;
use serde_json;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use csv::Reader;
use csv;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    token: String,
    customer_id: String,
    operators: i32,
}


#[derive(Debug,Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

pub fn run(data: &Data) {
    match load_config(data) {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Severo error {:?}", e),
    };
}

pub fn run_with_config(data: &Data, config: &Config) {}

fn load_config(data: &Data) -> Result<Config, io::Error> {
    let config_file = match data.config {
        Some(ref config_file) => config_file.clone(),
        None => String::from("config.json"),
    };

    let config = load_config_from_file(&config_file)?;
    Ok(config)
}

fn load_config_from_file(file_path: &String) -> Result<Config, io::Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("File not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let c: Config = serde_json::from_str(buf.as_str())?;
    Ok(c)
}

fn read_from_file(file_path: &String) -> Result<Vec<Record>, csv::Error> {
    let path = Path::new(file_path);
    let mut rdr = Reader::from_path(path)?;
    let mut records: Vec<Record> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}
