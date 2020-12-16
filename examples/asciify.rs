//! An example for turning an image into ASCII
extern crate asciifyer;

use asciifyer::{convert_to_ascii, Dimension};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Please enter a path to an image");

    let dimensions = Dimension::new(50, 50);
    let ascii = convert_to_ascii(&path, Some(dimensions));

    println!("{}", ascii);
}
