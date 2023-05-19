use std::process;
use pngme::{Error, Result};
use pngme::commands::Args;

fn main() {
    if let Err(e) = Args::parse() {
        println!("error occurred: {}", e);
        process::exit(2);
    }
}