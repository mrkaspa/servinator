use std::io::{self, Write};
use futures::{Future, Stream, future};
use hyper::{self, Client};
use hyper::client::FutureResponse;
use tokio_core::reactor::Core;
use runner::Record;

pub fn do_reqs(records: &Vec<Record>) -> Result<(), _> {
    let mut core = Core::new()?;
    let futs = records
        .iter()
        .filter_map(|ref record| {
                        let client = Client::new(&core.handle());
                        match "http://httpbin.org/ip".parse() {
                            Ok(uri) => Some(client.get(uri)),
                            Err(_) => None,
                        }

                    });
    let work = future::join_all(futs);
    core.run(work)?;
    Ok(())
}
