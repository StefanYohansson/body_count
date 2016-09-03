extern crate hyper;
extern crate encoding;
extern crate select;

pub mod crawler;

fn main() {
    crawler::request();
}
