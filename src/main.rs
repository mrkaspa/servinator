extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

pub mod cli;
pub mod runner;
mod req;

use cli::{process_cli, Command};
use runner::run;

fn main() {
    let cmd = process_cli();
    match cmd {
        Command::Run(data) => run(&data),
    };
    println!("HASTA LA VISTA, BABYYY");
}
