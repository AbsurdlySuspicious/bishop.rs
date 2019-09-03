#[macro_use] extern crate custom_error;

use std::io;
use std::result;

pub mod bishop;
mod vec2d;
mod input;

pub type Result<T> = result::Result<T, BsError>;

custom_error! {pub BsError
    Io{source: io::Error} = "IO: {source}",
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Err{msg: String} = "{msg}"
}

pub fn raise<R, S: Into<String>>(m: S) -> Result<R> {
    Err(BsError::Err { msg: m.into() })
}
