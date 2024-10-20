mod input_data;

#[macro_use]
extern crate custom_error;

use bishop::{errors::Error as BishopError, bishop_art::DEFAULT_CHARS, *};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use clap;
use clap::Parser as _;

use input_data::*;
use InputType::*;

custom_error! { BishopCliError
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Io{source: io::Error} = "IO: {source}",
    Bishop{source: BishopError} = "{source}",
    Other{msg: String} = "{msg}"
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum InputType {
    Bin,
    Hex,
    Hash
}

#[derive(Debug)]
enum Input<'a> {
  StdIn,
  File(&'a PathBuf),
  Hex(&'a String),
}

fn _raise<R, S: Into<String>>(m: S) -> Result<R, BishopCliError> {
  Err(BishopCliError::Other { msg: m.into() })
}

/// Visualizes keys and hashes using OpenSSH's Drunken Bishop algorithm
#[derive(clap::Parser, Debug)]
#[command(name = "bishop-cli")]
struct Opts {
  /// Input file
  #[arg(short, name = "file", display_order = 100)]
  input: Option<PathBuf>,

  /// Use stdin as input, shorthand for `-i -`
  #[arg(short = 's', long = "stdin", display_order = 101)]
  input_stdin: bool,

  #[arg(
    short = 'I',
    name = "type",
    ignore_case = true,
    value_enum,
    display_order = 200,
    help = "\
    Input type for -i
 bin  - Treat as binary data (default)
 hex  - Treat as HEX data
 hash - Hash input file as binary and then visualize hash (sha256)
        Use this for large inputs
 "
  )]
  input_type: Option<InputType>,

  /// Hash input data (shorthand for -I hash)
  #[arg(short = 'H', display_order = 201)]
  hash_input: bool,

  /// Treat input data as HEX (shorthand for -I hex)
  #[arg(short = 'X', display_order = 202)]
  hex_input: bool,

  /// HEX input, should have even length
  #[arg(name = "hex")]
  hex: Option<String>,

  /// Don't echo hex input
  #[arg(short, long, display_order = 0)]
  quiet: bool,

  /// Custom char list: '[bg][char]...[start][end]'
  #[arg(long, display_order = 200)]
  chars: Option<String>,

  /// Field width
  #[arg(short, long, default_value = "17", display_order = 301)]
  width: usize,

  /// Field height
  #[arg(short, long, default_value = "9", display_order = 302)]
  height: usize,

  /// Top frame text
  #[arg(short, long, display_order = 401)]
  top: Option<String>,

  /// Bottom frame text
  #[arg(short, long, display_order = 402)]
  bot: Option<String>,
}

fn input_echo(h: &impl AsRef<str>) {
  println!("Fingerprint of:\n{}\n", h.as_ref());
}

fn art_from_read<R: Read>(
  mut r: R,
  t: &InputType,
  art: &mut BishopArt,
  quiet: bool,
) -> io::Result<u64> {
  match t {
    Bin => io::copy(&mut r, art),
    Hash => {
      let a = hash_input(&mut r)?;
      if !quiet {
        input_echo(&hex::encode(a));
      }
      io::copy(&mut a.as_ref(), art)
    }
    Hex => {
      let mut i = HexInput::new(r);
      io::copy(&mut i, art)
    }
  }
}

fn str_opt<'a>(s: &'a Option<String>, d: &'static str) -> &'a str {
  if let Some(s) = s {
    &s
  } else {
    d
  }
}

fn main_() -> Result<(), BishopCliError> {
  let o = Opts::parse();

  let draw_opts = DrawingOptions {
    chars: str_opt(&o.chars, DEFAULT_CHARS).chars().collect(),
    top_text: str_opt(&o.top, "").to_string(),
    bottom_text: str_opt(&o.bot, "").to_string(),
  };

  let mut art = BishopArt::with_size(o.width, o.height)?;

  let mut input_t_set = o.input_type.is_some();
  let mut input_t = *(o.input_type.as_ref().unwrap_or(&Bin));
  let quiet = o.quiet;
  let dash = PathBuf::from("-");

  let input_t_shorthand = match (o.hash_input, o.hex_input) {
      (true, false) => Some(InputType::Hash),
      (false, true) => Some(InputType::Hex),
      (true, true) => _raise("Input type conflict")?,
      _ => None,
  };
  if let Some(sh_input_type) = input_t_shorthand {
      if input_t_set {
          _raise("Input type conflict")?;
          unreachable!();
      }
      input_t_set = true;
      input_t = sh_input_type;
  }


  let input_f = match (o.input_stdin, &o.input, &o.hex) {
    (true, None, None) => Input::StdIn,
    (false, Some(i), None) if *i == dash => Input::StdIn,
    (false, Some(i), None) => Input::File(i),
    (false, None, Some(h)) if !input_t_set => Input::Hex(h),
    _ => {
      _raise(
        "Either `(-s | -i <file>) [-I <type>]` _or_ `<hex>` should be passed\n\
         See --help for details",
      )?;
      unreachable!()
    }
  };

  match input_f {
    Input::StdIn => {
      let bf = io::stdin();
      art_from_read(bf.lock(), &input_t, &mut art, quiet)?;
    }
    Input::File(i) => {
      let f = File::open(i)?;
      let bf = BufReader::new(f);
      art_from_read(bf, &input_t, &mut art, quiet)?;
    }
    Input::Hex(h) => {
      if !quiet {
        input_echo(&h);
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

  use rand::prelude::*;
  use sha2::{Digest, Sha256};

  use std::error::Error as StdError;
  type StdResult = Result<(), Box<dyn StdError>>;

  const QUIET: bool = true;

  #[test]
  fn test_hash_input() -> StdResult {
    let mut data = [0u8; 64];
    thread_rng().fill_bytes(&mut data);

    let ref_hash: [u8; 32] = Sha256::new().chain(data.as_ref()).result().into();
    let ref_art = BishopArt::new().chain(ref_hash).draw();

    let mut art_inst = BishopArt::new();
    art_from_read(data.as_ref(), &InputType::Hash, &mut art_inst, QUIET)?;
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
    art_from_read(hex.as_bytes(), &InputType::Hex, &mut art_inst, QUIET)?;
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
    art_from_read(data.as_ref(), &InputType::Bin, &mut art_inst, QUIET)?;
    let chk_art = art_inst.draw();

    assert_eq!(ref_art, chk_art);
    Ok(())
  }
}
