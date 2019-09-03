use std::env::*;
use drunken_bishop::bishop::*;

fn main() {
    let emp = String::new();
    let argv: Vec<_> = args().collect();

    if argv.len() >= 2 {

        let inp = &argv[1];
        let top = argv.get(2).unwrap_or(&emp);
        let bot = argv.get(3).unwrap_or(&emp);

        let cfg = Options::new(DEFAULT_SIZE_WH, DEFAULT_CHARS, top, bot).unwrap();

        let data = hex::decode(inp).unwrap();

        println!("Input: {}\n", inp);
        println!("{}", art_str(&mut slice_input(&data), &cfg).unwrap());
        println!();
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
