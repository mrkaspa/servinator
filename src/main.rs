extern crate clap;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod cli;
mod runner;

use cli::{process_cli, Command};
use runner::run;

fn main() {
    let cmd = process_cli();
    match cmd {
        Command::Run(data) => run(&data),
    };
    println!("HASTA LA VISTA, BABY");
}
