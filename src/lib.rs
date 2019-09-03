#[macro_use] extern crate custom_error;

use std::io;

pub mod bishop;
mod vec2d;
mod input;

custom_error! {pub BsError
    Io{source: io::Error} = "IO: {source}",
    Hex{source: hex::FromHexError} = "Hex parse: {source}",
    Err{msg: String} = "Error: {msg}"
}
