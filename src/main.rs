#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![allow(dead_code)]
#![allow(unused_variables)]

//#![warn(clippy::all)]

mod error;
pub mod path;
mod infrastructure;
mod business;

type Result<T> = std::result::Result<T, error::Error>;

use error::Error;
use business::port;


fn main() {
    println!("Hello, world!");
}
