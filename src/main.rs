use drunken_bishop::{BsError, bishop::*};
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::{Read, BufReader};
use std::fs::File;

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

fn test() -> Result<(), BsError> {
    let o = Opts::from_args();
    println!("{:#?}", o);

    let cfg = Options::default();

    println!("hex art_str");
    let h = "aec070645fe53ee3b3763059376134f058cc337247c978add178b6ccdfb0019f";
    let hd = hex::decode(h)?;
    println!("{}\n", art_str(hd.as_slice(), &cfg)?);

    println!("file art_print");
    let f = File::open("test_stuff/foobar_hash")?;
    let mut bf = BufReader::new(f).bytes();
    art_print(& mut bf, &cfg, |s| println!("{}", s))?;
    println!();

    Ok(())
}

fn main_() -> Result<(), BsError> {
    test()
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("{}", e);
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
