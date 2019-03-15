use hyper::rt::{self, Future, Stream};
use hyper::{Client, Uri};

use clap::{App, Arg};
// use std::fs::File;

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
    println!("{}", outfile);

    // クライアントを作成
    let client = Client::new();

    // クライアントのFutureを作成
    let fut = client
        .get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());
            // bodyのstreamを繋げて新しいFutureをつくる
            res.into_body().concat2()
        })
        .and_then(|_body| {
            // match File::create(&outfile) {
            //     Err(_) => {}
            //     Ok(_) => {}
            // }
            Ok(())
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });

    rt::run(fut);
}
