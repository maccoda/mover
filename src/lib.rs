extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use futures::Future;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use serde_json::Value;

use std::net::SocketAddr;

mod db;
mod uri;
use db::Database;
use uri::UriPath;

struct Server {
    db: db::Database,
}

fn build_path(req: &Request) -> UriPath {
    let query = req.query().unwrap_or("");
    UriPath::new(format!(
        "{}?{}",
        &req.path().chars().skip(1).collect::<String>(),
        query
    ))
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
        println!("Received {} request for {}", req.method(), req.path());

        let response = match req.method() {
            Method::Get => retrieve_from_db(&req, &self.db),
            _ => Response::new().with_status(StatusCode::NotFound),
        };
        Box::new(futures::future::ok(response))
    }
}

fn retrieve_from_db(req: &Request, db: &Database) -> Response {
    let mut response = Response::new();
    let path = build_path(req);
    if let Some(return_data) = extract_data_from_db(&path, db) {
        response.set_body(
            serde_json::to_string(&return_data).expect("Failed to convert JSON to string"),
        );
    } else {
        response.set_status(StatusCode::NotFound)
    }
    response.with_header(hyper::header::ContentType::json())
}

fn extract_data_from_db(path: &UriPath, db: &Database) -> Option<Value> {
    let key = path.root();
    if path.id_segment().is_some() {
        db.find_with_id(key, path.id_segment().unwrap())
    } else {
        let mut value = db.get(key);
        for (key, val) in path.query_params() {
            value = value.map(|x| filter_json_array(&x, key, val));
        }
        value
    }
}

fn filter_json_array(arr: &Value, filter_key: &str, filter_value: &str) -> Value {
    if arr.is_array() {
        let arr = arr.as_array().expect("Can only filter on an array");
        let vec_vals: Vec<Value> = arr
            .into_iter()
            .filter(|x| {
                let object_value = x.as_object().expect("Not an object").get(filter_key);
                if let Some(key) = object_value {
                    key == filter_value
                } else {
                    true
                }
            }).map(|x| x.to_owned())
            .collect();
        Value::Array(vec_vals)
    } else {
        arr.to_owned()
    }
}

pub fn start(db: Database, addr: &SocketAddr) {
    println!("{:#?}", db);
    let server = Http::new()
        .bind(&addr, move || Ok(Server { db: db.clone() }))
        .unwrap();
    let socket_addr = server.local_addr().unwrap();
    println!("Listening on {:?}", socket_addr);
    server.run().unwrap();
}
