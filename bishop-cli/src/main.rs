mod input_data;

#[macro_use] extern crate custom_error;
use bishop::{bishop::*, _raise, BishopError};
use structopt::StructOpt;
use structopt::clap::arg_enum;
use std::path::PathBuf;
use std::io::{self, Read, BufReader};
use std::fs::File;

use input_data::*;

custom_error!{ BishopCliError
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Io{source: io::Error} = "IO: {source}",
    Other{source: BishopError} = "{source}"
}

arg_enum! {
    #[derive(Debug)]
    enum InputType {
        Bin,
        Hex,
        Hash
    }
}

use InputType::*;

/// Visualizes keys and hashes using OpenSSH's Drunken Bishop algorithm
#[derive(StructOpt, Debug)]
#[structopt(name = "bishop-cli")]
struct Opts {

    /// Input file; use '-' for stdin
    #[structopt(short, name = "input-data", parse(from_os_str), display_order = 100)]
    input: Option<PathBuf>,

    #[structopt(short = "T", case_insensitive = true, display_order = 101, help = "\
    Input type for -i
 'bin'  - Binary data (default)
 'hex'  - Hex data
 'hash' - Hash binary input and then visualize hash
 ")]
    input_type: Option<InputType>,

    /// Hex input; should have even length
    #[structopt(name = "hex")]
    hex: Option<String>,

    /// Don't echo hex input
    #[structopt(short, long, display_order = 0)]
    quiet: bool,

    /// Custom char list: '[bg][char]...[start][end]'
    #[structopt(long, display_order = 200)]
    chars: Option<String>,

    /// Field width
    #[structopt(short, long, default_value = "17", display_order = 301)]
    width: usize,

    /// Field height
    #[structopt(short, long, default_value = "9", display_order = 302)]
    height: usize,

    /// Top frame text
    #[structopt(short, long, display_order = 401)]
    top: Option<String>,

    /// Bottom frame text
    #[structopt(short, long, display_order = 402)]
    bot: Option<String>,
}

fn print(s: &String) {
    println!("{}", s);
}

fn p_input<R: Read>(r: R, t: &InputType, cfg: &Options) -> Result<(), BishopError> {
    match t {
        Hash => unimplemented!(),
        Bin => art_print(&mut r.bytes(), cfg, print),
        Hex => {
            let i = InputItself(HexInput::new(r));
            art_print(i, cfg, print)
        }
    }
}

fn str_opt<'a>(s: &'a Option<String>, d: &'static str) -> &'a str {
    if let Some(s) = s { &s } else { d }
}

fn main_() -> Result<(), BishopCliError> {
    let o = Opts::from_args();

    println!("{:#?}", o);
    //_raise("heh")?;

    let cfg = Options::new(
        (o.width, o.height),
        str_opt(&o.chars, DEFAULT_CHARS),
        str_opt(&o.top, ""),
        str_opt(&o.bot, "")
    )?;

    let input_t_set = o.input_type.is_some();
    let input_t = o.input_type.unwrap_or(Bin);
    let dash = PathBuf::from("-");

    let () = match (&o.input, o.hex) {
        (Some(d), None) if *d == dash => {
            let bf = io::stdin();
            p_input(bf.lock(), &input_t, &cfg)?
        },
        (Some(i), None) => {
            let f = File::open(i)?;
            let bf = BufReader::new(f);
            p_input(bf, &input_t, &cfg)?
        },
        (None, Some(h)) => {
            if input_t_set {
                _raise("-T can be used with -i only")?
            }

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
