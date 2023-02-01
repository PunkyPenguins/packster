#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![allow(dead_code)]
#![allow(unused_variables)]

//#![warn(clippy::all)]

mod error;
pub mod path;
mod infrastructure;
mod essential;
mod application;

type Result<T> = std::result::Result<T, error::Error>;

use error::Error;
use essential::port;


fn main() {
    println!("Hello, world!");
}
