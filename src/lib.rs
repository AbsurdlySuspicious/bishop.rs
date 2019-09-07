#[macro_use] extern crate custom_error;

use std::io;
use std::result;

pub mod bishop;
pub mod input;
mod vec2d;

pub type Result<T> = result::Result<T, BishopError>;

custom_error! {pub BishopError
    Io{source: io::Error} = "IO: {source}",
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Err{msg: String} = "{msg}"
}

pub fn _raise<R, S: Into<String>>(m: S) -> Result<R> {
    Err(BishopError::Err { msg: m.into() })
}
