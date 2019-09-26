mod input_data;

#[macro_use]
extern crate custom_error;

use bishop::{*, bishop_art::DEFAULT_CHARS};
use structopt::StructOpt;
use structopt::clap::arg_enum;
use std::path::PathBuf;
use std::io::{self, Read, BufReader};
use std::fs::File;

use input_data::*;

custom_error! { BishopCliError
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Io{source: io::Error} = "IO: {source}",
    Bishop{source: BishopError} = "{source}",
    Other{msg: String} = "{msg}"
}

arg_enum! {
    #[derive(Debug)]
    enum InputType {
        Bin,
        Hex,
        Hash
    }
}

#[derive(Debug)]
enum Input<'a> {
    StdIn,
    File(&'a PathBuf),
    Hex(&'a String)
}

use InputType::*;

fn _raise<R, S: Into<String>>(m: S) -> Result<R, BishopCliError> {
    Err(BishopCliError::Other { msg: m.into() })
}

/// Visualizes keys and hashes using OpenSSH's Drunken Bishop algorithm
#[derive(StructOpt, Debug)]
#[structopt(name = "bishop-cli")]
struct Opts {
    /// Input file
    #[structopt(short, name = "file", parse(from_os_str), display_order = 100)]
    input: Option<PathBuf>,

    /// Use stdin as input, shorthand for `-i -`
    #[structopt(short = "s", long = "stdin", display_order = 101)]
    input_stdin: bool,

    #[structopt(short = "I", name = "type",
    case_insensitive = true, display_order = 102, help = "\
    Input type for -i
 bin  - Treat as binary data (default)
 hex  - Treat as HEX data
 hash - Hash input file as binary and then visualize hash (sha256)
        Use this for large inputs
 ")]
    input_type: Option<InputType>,

    /// HEX input, should have even length
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

fn art_from_read<R: Read>(mut r: R, t: &InputType, art: &mut BishopArt) -> io::Result<u64> {
    match t {
        Bin => io::copy(&mut r, art),
        Hash => {
            let a = hash_input(&mut r)?;
            io::copy(&mut a.as_ref(), art)
        }
        Hex => {
            let mut i = HexInput::new(r);
            io::copy(&mut i, art)
        }
    }
}

fn str_opt<'a>(s: &'a Option<String>, d: &'static str) -> &'a str {
    if let Some(s) = s { &s } else { d }
}

fn main_() -> Result<(), BishopCliError> {
    let o = Opts::from_args();

    let draw_opts = DrawingOptions {
        chars: str_opt(&o.chars, DEFAULT_CHARS).chars().collect(),
        top_text: str_opt(&o.top, "").to_string(),
        bottom_text: str_opt(&o.bot, "").to_string(),
    };

    let mut art = BishopArt::with_size(o.width, o.height)?;

    let input_t_set = o.input_type.is_some();
    let input_t = o.input_type.unwrap_or(Bin);
    let dash = PathBuf::from("-");

    let input_f = match (o.input_stdin, &o.input, &o.hex) {
        (true, None, None) => Input::StdIn,
        (false, Some(i), None) if *i == dash => Input::StdIn,
        (false, Some(i), None) => Input::File(i),
        (false, None, Some(h)) if !input_t_set => Input::Hex(h),
        _ => {
            _raise(
                "Either `(-s | -i <file>) [-I <type>]` _or_ `<hex>` should be passed\n\
                See --help for details")?;
            unreachable!()
        }
    };

    match input_f {
        Input::StdIn => {
            let bf = io::stdin();
            art_from_read(bf.lock(), &input_t, &mut art)?;
        }
        Input::File(i) => {
            let f = File::open(i)?;
            let bf = BufReader::new(f);
            art_from_read(bf, &input_t, &mut art)?;
        }
        Input::Hex(h) => {
            if !o.quiet {
                println!("Fingerprint of:\n{}\n", h);
            }

            let d = hex::decode(h)?;
            art.input(d);
        }
    };

    let res = art.draw_with_opts(&draw_opts);
    print!("{}", res);

    Ok(())
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sha2::{Sha256, Digest};
    use rand::prelude::*;

    use std::error::{Error as StdError};
    type StdResult = Result<(), Box<dyn StdError>>;

    #[test]
    fn test_hash_input() -> StdResult {
        let mut data = [0u8; 64];
        thread_rng().fill_bytes(&mut data);

        let ref_hash: [u8; 32] = Sha256::new().chain(data.as_ref()).result().into();
        let ref_art = BishopArt::new().chain(ref_hash).draw();

        let mut art_inst = BishopArt::new();
        art_from_read(data.as_ref(), &InputType::Hash, &mut art_inst)?;
        let chk_art = art_inst.draw();

        assert_eq!(ref_art, chk_art);
        Ok(())
    }

    #[test]
    fn test_hex_input() -> StdResult {
        let mut data = [0u8; 32];
        thread_rng().fill_bytes(&mut data);

        let ref_art = BishopArt::new().chain(data).draw();
        let hex = hex::encode(data.as_ref());

        let mut art_inst = BishopArt::new();
        art_from_read(hex.as_bytes(), &InputType::Hex, &mut art_inst)?;
        let chk_art = art_inst.draw();

        assert_eq!(ref_art, chk_art);
        Ok(())
    }

    #[test]
    fn test_bin_input() -> StdResult {
        let mut data = [0u8; 32];
        thread_rng().fill_bytes(&mut data);

        let ref_art = BishopArt::new().chain(data).draw();

        let mut art_inst = BishopArt::new();
        art_from_read(data.as_ref(), &InputType::Bin, &mut art_inst)?;
        let chk_art = art_inst.draw();

        assert_eq!(ref_art, chk_art);
        Ok(())
    }

}

