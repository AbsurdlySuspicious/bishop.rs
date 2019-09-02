use drunken_bishop::*;
use std::env::*;

fn main() {
    let emp = String::new();
    let argv: Vec<_> = args().collect();

    if argv.len() >= 2 {
        let inp = &argv[1];
        let top = argv.get(2).unwrap_or(&emp);
        let bot = argv.get(3).unwrap_or(&emp);
        println!("Input: {}\n", inp);
        bishop::heh(inp, top, bot);
        println!();
    } else {
        bishop::heh2();
    }
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
