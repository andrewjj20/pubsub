#![feature(rust_2018_preview, use_extern_macros)]

#[macro_use]
extern crate log;

use clap::{crate_authors, crate_version, App as ClApp, Arg};
use futures::{Future, Stream};
use pubsub::publisher::Publisher;

fn main() {
    env_logger::init();

    let matches = ClApp::new("stdin_publisher")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let name = "stdin".to_string();
    let host_name = matches.value_of("host").unwrap().to_string();
    let port = matches
        .value_of("port")
        .unwrap()
        .parse()
        .expect("invalid integer in port");
    tokio::run(
        Publisher::new(name, host_name, port, url)
            .map_err(|e| {
                error!("Creating Publisher {}", e);
            })
            .and_then(|p| {
                tokio_codec::FramedRead::new(tokio::io::stdin(), tokio_codec::LinesCodec::new())
                    .map_err(|e| pubsub::Error::from(e))
                    .map(|s| Vec::from(s))
                    .forward(p)
                    .map(|_| ())
                    .map_err(|e| {
                        error!("Error writing to publisher {}", e);
                    })
            }),
    );
}