#[macro_use] extern crate custom_error;

use std::io;

pub mod bishop;
mod vec2d;

pub use bishop::{BishopArt, BishopResult, DrawingOptions};

custom_error! {pub BishopError
    Io{source: io::Error} = "IO: {source}",
    Err{msg: String} = "{msg}"
}

fn _raise<R, S: Into<String>>(m: S) -> result::Result<R> {
    Err(BishopError::Err { msg: m.into() })
}

pub mod result {
    use std::result;
    use crate::BishopError;

    /// Local result type
    pub type Result<T> = result::Result<T, BishopError>;
}
