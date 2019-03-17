use hyper::rt::{self, Future, Stream};
use hyper::{Client, StatusCode, Uri};

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

fn fetch_uri(uri: Uri, mut file: File) -> impl Future<Item = (), Error = ()> {
    // クライアントを作成
    let client = Client::new();

    // クライアントのFutureを作成
    client
        .get(uri)
        .and_then(move |res| {
            if res.status() != StatusCode::OK {
                panic!("{}", res.status());
            }

            res.into_body()
                .for_each(move |chunk| file.write_all(&chunk).map_err(|e| panic!("{}", e)))
        })
        .map(|_| {
            println!("done");
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

fn main() {
    // オプション処理
    let matches = App::new("rwget")
        .arg(Arg::with_name("URI").required(true))
        .arg(
            Arg::with_name("file")
                .short("O")
                .long("output-document")
                .help("The documents will not be written to the appropriate files, but all will be concatenated together and written to file")
                .takes_value(true),
        )
        .get_matches();

    // URIを解析
    let uri: Uri = matches
        .value_of("URI")
        .expect("no URI")
        .parse()
        .expect("invalid URI");

    // 出力ファイル名
    let outfile = matches
        .value_of("file")
        .unwrap_or(if uri.path() == "/" {
            "index.html"
        } else {
            uri.path()
        })
        .to_string();

    let file = match File::create(&outfile) {
        Err(_) => {
            panic!("couldn't open {}", outfile);
        }
        Ok(file) => file,
    };

    let fut = fetch_uri(uri, file);

    // ランタイムで実行
    rt::run(fut);
}
