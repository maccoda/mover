/// All tests to provide the basic json_server implementation
extern crate mover;
extern crate reqwest;
#[macro_use]
extern crate serde_json;

use std::thread;
use std::sync::{Once, ONCE_INIT};
use serde_json::Value;
use reqwest::Url;

static START: Once = ONCE_INIT;

fn serve() {
    let db = serde_json::from_value(json!(
        {
        "posts": [{ "id": 1, "title": "mover", "author": "maccoda" }],
        "comments": [{ "id": 1, "body": "some comment", "postId": 1 }],
        "profile": { "name": "maccoda" }
        }
    )).unwrap();
    START.call_once(|| {
        thread::spawn(move || mover::start(db, &"127.0.0.1:5212".parse().unwrap()));
    });
}

fn get_request(path: &str) -> Value {
    let url = Url::parse("http://localhost:5212/")
        .unwrap()
        .join(path)
        .unwrap();
    let req = reqwest::get(url).unwrap();
    assert_eq!(req.status(), reqwest::StatusCode::Ok);
    let mut req = req;
    req.json().expect("Failed to decode response")
}

#[test]
fn plural_get_all() {
    serve();
    let actual = get_request("posts");
    assert_eq!(
        json!([{"id":1, "title": "mover", "author": "maccoda"}]),
        actual
    );
}

#[test]
fn plural_get_one() {
    serve();
    let actual = get_request("posts/1");
    assert_eq!(
        json!({"id":1, "title": "mover", "author": "maccoda"}),
        actual
    );
}

#[test]
fn singular_get() {
    serve();
    let actual = get_request("profile");
    assert_eq!(json!({"name": "maccoda"}), actual);
}
