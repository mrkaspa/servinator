use serde_json;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use csv::Reader;
use csv;
use structs::{Data, Config, Task};
use req::do_reqs;

pub fn run(data: &Data) -> Result<(), io::Error> {
    let config = load_config(data)?;
    run_with_config(data, &config)
}

pub fn run_with_config(data: &Data, config: &Config) -> Result<(), io::Error> {
    for entry in fs::read_dir(&data.dir)? {
        let entry = entry?;
        match read_from_file(&entry.path()) {
            Ok(records) => do_reqs(config, &records).unwrap(),
            Err(err) => println!("err csv {:?}", err),
        };
    }
    Ok(())
}

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

fn read_from_file(path: &Path) -> Result<Vec<Task>, csv::Error> {
    let mut rdr = Reader::from_path(path)?;
    let mut records: Vec<Task> = Vec::new();
    for result in rdr.deserialize() {
        let record: Task = result?;
        records.push(record);
    }
    Ok(records)
}
