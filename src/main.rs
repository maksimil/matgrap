use std::env;

pub mod canvas;

pub use canvas::*;

fn main() {
    let args: Vec<String> = env::args().collect();
}
