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
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database(HashMap<String, Value>);

#[derive(Debug)]
struct UriPath {
    parts: Vec<String>,
}

impl UriPath {
    fn new(path: &str) -> UriPath {
        let parts = path.split("/").map(|x| x.to_owned()).collect();
        UriPath { parts }
    }

    fn root(&self) -> &str {
        &self.parts[0]
    }

    fn part(&self, index: usize) -> &str {
        &self.parts[index]
    }

    fn len(&self) -> usize {
        self.parts.len()
    }
}

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
                let path = UriPath::new(&req.path().chars().skip(1).collect::<String>());
                let key = path.root();
                let element = self.db.0.get(key).expect("Key not in database");
                let result = if path.len() > 1 {
                    let id: usize = path.part(1)
                        .parse()
                        .expect("Second path element is not a number");
                    &element
                        .as_array()
                        .expect("Not an array")
                        .iter()
                        .filter(|x| {
                            x.as_object()
                                .expect("Not an object")
                                .get("id")
                                .expect("No ID field found")
                                .as_u64()
                                .expect("ID not u64") == id as u64
                        })
                        .next()
                        .expect("No matching ID")
                } else {
                    element
                };
                println!("Retrieving for {:?}", key);
                response.set_body(
                    serde_json::to_string(result).expect("Failed to convert JSON to string"),
                );
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

pub fn start(db: Database, addr: &SocketAddr) {
    println!("{:?}", db);
    let server = Http::new()
        .bind(&addr, move || Ok(Server { db: db.clone() }))
        .unwrap();
    let socket_addr = server.local_addr().unwrap();
    println!("Listening on {:?}", socket_addr);
    server.run().unwrap();
}
