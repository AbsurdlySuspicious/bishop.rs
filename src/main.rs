use drunken_bishop::{bishop::*, _raise, Result};
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::{self, Read, BufReader};
use std::fs::File;

/// Visualizes keys and hashes using OpenSSH's Drunken Bishop algorithm
#[derive(StructOpt, Debug)]
#[structopt(name = "drunken-bishop")]
struct Opts {

    /// Input file; use '-' for stdin
    #[structopt(short, parse(from_os_str), display_order = 1)]
    input: Option<PathBuf>,

    /// Hex input; should have even length
    #[structopt(name = "hex")]
    hex: Option<String>,

    /// Don't echo hex input
    #[structopt(short, long, display_order = 0)]
    quiet: bool,

    /// Custom char list: '[bg][char]...[start][end]'
    #[structopt(long, display_order = 2)]
    chars: Option<String>,

    /// Field width
    #[structopt(short, long, default_value = "17", display_order = 3)]
    width: usize,

    /// Field height
    #[structopt(short, long, default_value = "9", display_order = 4)]
    height: usize,

    /// Top frame text
    #[structopt(short, long, display_order = 5)]
    top: Option<String>,

    /// Bottom frame text
    #[structopt(short, long, display_order = 6)]
    bot: Option<String>,
}

fn str_opt<'a>(s: &'a Option<String>, d: &'static str) -> &'a str {
    if let Some(s) = s { &s } else { d }
}


fn main_() -> Result<()> {
    let o = Opts::from_args();

    let cfg = Options::new(
        (o.width, o.height),
        str_opt(&o.chars, DEFAULT_CHARS),
        str_opt(&o.top, ""),
        str_opt(&o.bot, "")
    )?;

    let print = |s: &String| println!("{}", s);
    let dash = PathBuf::from("-");

    let () = match (&o.input, o.hex) {
        (Some(d), None) if *d == dash => {
            let mut bf = io::stdin().bytes();
            art_print(&mut bf, &cfg, print)?
        },
        (Some(i), None) => {
            let f = File::open(i)?;
            let mut bf = BufReader::new(f).bytes();
            art_print(&mut bf, &cfg, print)?
        },
        (None, Some(h)) => {
            if !o.quiet {
                println!("Fingerprint of:\n{}\n", h);
            }

            let d = hex::decode(h)?;
            art_print(d.as_slice(), &cfg, print)?
        },
        _ => _raise(
            "Either -i OR <hex> should be specified\n\
            Check --help for usage")?
    };

    Ok(())
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("{}", e);
        std::process::exit(1);
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
