extern crate mover;
extern crate serde_json;
extern crate structopt;

use std::fs::File;
use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mover")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value="5212")]
    port: usize,

    #[structopt(short = "d", long = "database", default_value = "db.json")]
    database_file: String
}

fn main() {
    let opt = Opt::from_args();

    let mut content = String::new();
    File::open(&opt.database_file).and_then(|mut f| f.read_to_string(&mut content))
        .expect(&format!("Unable to read {}", &opt.database_file));

    let url = &format!("127.0.0.1:{}", opt.port).parse().unwrap();

    mover::start(
        serde_json::from_str(&content).expect("Failed to parse database JSON"),
        url,
    );
}
