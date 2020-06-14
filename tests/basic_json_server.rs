/// All tests to provide the basic json_server implementation
extern crate mover;
extern crate reqwest;
#[macro_use]
extern crate serde_json;

use reqwest::{Response, Url};
use serde_json::Value;
use std::sync::Once;
use std::thread;

static START: Once = Once::new();

fn serve() {
    let db = serde_json::from_value(json!(
        {
        "posts": [
            { "id": 1, "title": "mover", "author": "maccoda" },
            {"id": 2, "title": "web stuff", "author": "bob"}
        ],
        "comments": [{ "id": 1, "body": "some comment", "postId": 1 }],
        "profile": { "name": "maccoda" }
        }
    )).unwrap();
    START.call_once(|| {
        thread::spawn(move || mover::start(db, &"127.0.0.1:5212".parse().unwrap()));
    });
}

fn get_request(path: &str) -> Response {
    let url = Url::parse("http://localhost:5212/")
        .unwrap()
        .join(path)
        .unwrap();
    reqwest::get(url).unwrap()
}

fn json_get_request(path: &str) -> Value {
    let req = get_request(path);
    assert_eq!(req.status(), reqwest::StatusCode::OK);
    let mut req = req;
    req.json().expect("Failed to decode response")
}

#[test]
fn plural_get_all() {
    serve();
    let actual = json_get_request("posts");
    assert_eq!(
        json!(
            [   {"id":1, "title": "mover", "author": "maccoda"},
                {"id": 2, "title": "web stuff", "author": "bob"}
            ]
            ),
        actual
    );
}

#[test]
fn plural_get_one() {
    serve();
    let actual = json_get_request("posts/1");
    assert_eq!(
        json!({"id":1, "title": "mover", "author": "maccoda"}),
        actual
    );
}

#[test]
fn singular_get() {
    serve();
    let actual = json_get_request("profile");
    assert_eq!(json!({"name": "maccoda"}), actual);
}

#[test]
fn query_parameter_get_should_filter() {
    serve();
    let actual = json_get_request("posts?author=maccoda");
    assert_eq!(
        json!([{"id":1, "title": "mover", "author": "maccoda"}]),
        actual
    );
}

#[test]
fn query_parameter_get_no_results() {
    serve();
    let actual = json_get_request("posts?author=frank");
    assert_eq!(json!([]), actual)
}

#[test]
fn query_parameter_get_query_key_invalid_should_ignore() {
    serve();
    let actual = json_get_request("posts?auth=bob");
    assert_eq!(
        json!(
            [   {"id":1, "title": "mover", "author": "maccoda"},
                {"id": 2, "title": "web stuff", "author": "bob"}
            ]
            ),
        actual
    );
}

#[test]
fn query_parameter_get_on_singular_item_should_be_ignored() {
    serve();
    let actual = json_get_request("profile?name=bob");
    assert_eq!(json!({"name": "maccoda"}), actual)
}

#[test]
fn not_found_for_unknown_path() {
    serve();
    let actual = get_request("unknown");
    assert_eq!(actual.status(), reqwest::StatusCode::NOT_FOUND);
}

#[test]
fn not_found_for_not_present_id() {
    serve();
    let actual = get_request("posts/123");
    assert_eq!(actual.status(), reqwest::StatusCode::NOT_FOUND);
}
