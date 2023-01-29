mod error;
mod mandatory;
mod essential;

type Result<T> = std::result::Result<T, error::Error>;

use error::Error;
use essential::port;

fn main() {
    println!("Hello, world!");
}
