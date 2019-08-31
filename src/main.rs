use drunken_bishop::*;
use std::env::*;

fn main() {
    let argv: Vec<_> = args().collect();
    let inp = &argv[1];
    println!("Input: {}\n", inp);
    bishop::heh(inp);
}

// binary from file:
// $ bishop -i file
//
// binary from stdin
// $ bishop -i -
// $ bishop -c
//
// hex from arg
// $ bishop 'aec070645fe53ee3b37630'
