use std::env;

pub mod canvas;
pub mod parser;

pub use canvas::*;
pub use parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
}
