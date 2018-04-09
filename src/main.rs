extern crate mover;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut content = String::new();
    let mut file = File::open("db.json").expect("Unable to open db.json");
    file.read_to_string(&mut content)
        .expect("Unable to read db.json");
    mover::start(serde_json::from_str(&content).expect("Unable to convert to type"))
}
