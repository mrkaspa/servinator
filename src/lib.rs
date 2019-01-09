extern crate clap;
extern crate csv;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

pub mod cli;
mod req;
pub mod runner;
pub mod structs;
