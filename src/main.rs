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
pub mod structs;
mod req;

use cli::process_cli;
use structs::{Command, Task};
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
    };
}

// fn main() {
//     let vr = vec![Task { city: String::from("eooo") },
//                   Task { city: String::from("eooo") }];
//     req::do_reqs(&vr);
// }
