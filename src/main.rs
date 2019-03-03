extern crate clap;
extern crate hyper;

use hyper::rt::{self, Future}; // Stream
use hyper::{Client, Uri};
// use std::io::{self, Write};

use clap::{App, Arg};

fn main() {
    let matches = App::new("rwget")
        .arg(Arg::with_name("URI").required(true))
        .get_matches();

    let uri: Uri = matches
        .value_of("URI")
        .expect("no URI")
        .parse()
        .expect("invalid URI");

    rt::run(rt::lazy(|| {
        let client = Client::new();
        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));
}
