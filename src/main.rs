use std::env::*;
use drunken_bishop::bishop::*;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "drunken-bishop")]
struct Opts {

    /// Input file; use - for stdin
    #[structopt(short, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Hex input; should have even length
    #[structopt(name = "hex")]
    hex: Option<String>,

    /// Don't repeat input to console
    #[structopt(short, long)]
    quiet: bool,

    /// Custom char list: '[bg][char]...[start][end]'
    #[structopt(long)]
    chars: Option<String>,

    /// Field width
    #[structopt(short, long, default_value = "17")]
    width: u8,

    /// Field height
    #[structopt(short, long, default_value = "9")]
    height: u8,

    /// Top frame text
    #[structopt(short, long)]
    top: Option<String>,

    /// Bottom frame text
    #[structopt(short, long)]
    bot: Option<String>,
}

fn main() {
    let o = Opts::from_args();
    println!("{:#?}", o);
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
