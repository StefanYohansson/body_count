#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate encoding;
extern crate select;
extern crate postgres;
#[macro_use]
extern crate rustless;

extern crate iron;
extern crate url;

extern crate valico;

pub mod connection;
pub mod crawler;
pub mod api;
pub mod models;

use docopt::Docopt;

const USAGE: &'static str = "
Body Count: a list or total of casualties.

Usage:
  body_count (-w | --http)
  body_count (-s | --crawler)
  body_count (-h | --help)
  body_count --version

Options:
  -w --http       Start API.
  -d --deamon     Start API as daemon.
  -s --crawler    Get data from ITEP.
  -h --help       Show this screen.
  --version       Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_http: bool,
    flag_crawler: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_crawler {
        crawler::request();
    }

    if args.flag_http {
        api::start();
    }
}
