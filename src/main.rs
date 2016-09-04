extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate encoding;
extern crate select;
extern crate postgres;

pub mod connection;
pub mod crawler;

use docopt::Docopt;

const USAGE: &'static str = "
Body Count: a list or total of casualties.

Usage:
  body_count_rn (-w | --http)
  body_count_rn (-s | --crawler)
  body_count_rn (-h | --help)
  body_count_rn --version

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
        println!("http {:?}", args);
    }
}
