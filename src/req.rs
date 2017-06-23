use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType, Authorization};
use tokio_core::reactor::Core;
use serde_json;
use structs::{Task, Config, Service};

pub fn do_reqs(config: &Config, records: &Vec<Task>) -> Result<(), serde_json::Error> {
    println!("conf = {:?}", config);
    println!("rec = {:?}", records);
    match config.url.parse() {
        Ok(uri) => {
            let service = Service {
                service_type_id: config.service_type_id,
                customer_id: config.customer_id,
                num_operators: config.operators,
                tasks: (*records).clone(),
            };
            let json = serde_json::to_string(&service)?;
            println!("json {:?}", json);

            let mut req = Request::new(Method::Post, uri);
            req.headers_mut().set(ContentType::json());
            req.headers_mut().set(ContentLength(json.len() as u64));
            req.headers_mut()
                .set(Authorization(config.token.clone()));
            req.set_body(json);
            println!("req {:?}", req);

            let mut core = Core::new().unwrap();
            let client = Client::new(&core.handle());
            let res = core.run(client.request(req)).unwrap();
            println!("res {:?}", res);
        }
        Err(_) => (),
    };
    Ok(())
}
