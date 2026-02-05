#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_late_init)]
#![allow(non_snake_case)]

#[macro_use]
extern crate num_derive;

pub mod diag;
pub mod h264;
pub mod y4m_cmp;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
