#![allow(unused)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]

use crate::h264::tables;

#[macro_use]
extern crate num_derive;

pub mod h264;

fn main() {
    println!("Table 9-5: \n {:#?}", tables::TABLE95);
}
