#[macro_use] extern crate custom_error;

use std::io;

pub mod bishop2;
pub mod result;
mod vec2d;

pub use bishop2::{BishopArt, BishopResult, DrawingOptions};

custom_error! {pub BishopError
    Io{source: io::Error} = "IO: {source}",
    Err{msg: String} = "{msg}"
}

fn _raise<R, S: Into<String>>(m: S) -> result::Result<R> {
    Err(BishopError::Err { msg: m.into() })
}
