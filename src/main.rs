use drunken_bishop::{bishop::*, raise, Result};
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::{self, Read, BufReader};
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
    width: usize,

    /// Field height
    #[structopt(short, long, default_value = "9")]
    height: usize,

    /// Top frame text
    #[structopt(short, long)]
    top: Option<String>,

    /// Bottom frame text
    #[structopt(short, long)]
    bot: Option<String>,
}

fn _test() -> Result<()> {
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
            eprintln!("stdin");
            let mut bf = io::stdin().bytes();
            art_print(&mut bf, &cfg, print)?
        },
        (Some(i), None) => {
            eprintln!("file");
            let f = File::open(i)?;
            let mut bf = BufReader::new(f).bytes();
            art_print(&mut bf, &cfg, print)?
        },
        (None, Some(h)) => {
            eprintln!("hex");
            if !o.quiet {
                println!("Fingerprint of {}\n", h);
            }

            let d = hex::decode(h)?;
            art_print(d.as_slice(), &cfg, print)?
        },
        _ => raise(
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
