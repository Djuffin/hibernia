#![allow(unused)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate num_derive;

pub mod h264;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {:?}", args);
    if args.len() > 1 {
        let name = args[1].clone();
        println!("Processing {}", name);
        let buf = fs::read(name).unwrap();
        let mut decoder = h264::Decoder::new();
        decoder.decode(&buf).expect("parsing error");
    }
}
