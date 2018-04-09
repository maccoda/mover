extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use futures::Future;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database(HashMap<String, Value>);

struct Server {
    db: Database,
}

impl Service for Server {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        println!("Received {} request for {}", req.method(), req.path());

        match req.method() {
            &Method::Get => {
                response.set_body("You will be getting some data soon");
            }
            &Method::Post => {
                // need to add the posting
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        };

        Box::new(futures::future::ok(response))
    }
}

pub fn start(db: Database) {
    println!("{:?}", db);
    let addr = "127.0.0.1:5212".parse().unwrap();
    let server = Http::new()
        .bind(&addr, move || Ok(Server { db: db.clone() }))
        .unwrap();
    println!("Listening on {}", addr);
    server.run().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
