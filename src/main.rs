extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

mod cli;
mod runner;
mod structs;
mod req;

use cli::process_cli;
use structs::Command;
use runner::run;

fn main() {
    let cmd = process_cli();
    match cmd {
        Command::Run(data) => {
            match run(&data) {
                Ok(_) => println!("HASTA LA VISTA, BABYYY"),
                Err(e) => println!("Severo error {:?}", e),
            }
        }
        Command::None => (),
    };
}
