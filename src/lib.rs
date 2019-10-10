//!
//! Crate for visualizing data using The Drunken Bishop algorithm
//!
//! > Drunken Bishop is the algorithm used in OpenSSH's `ssh-keygen` for visualising generated keys
//!
//! # Examples
//!
//! ## For `AsRef<u8>` (slices, vectors)
//!
//! ```
//! extern crate bishop;
//! use bishop::*;
//!
//! fn main() {
//!     let data1 = [0u8; 16];
//!     let data2 = vec![0u8; 16];
//!
//!     let mut art = BishopArt::new();
//!     art.input(&data1);
//!     art.input(&data2);
//!     println!("{}", art.draw());
//!
//!     // Using chaining:
//!
//!     let drawn_art: String = BishopArt::new()
//!         .chain(&data1)
//!         .chain(&data2)
//!         .draw();
//!     println!("{}", drawn_art);
//! }
//! ```
//!
//! ## Drawing options and result reusing
//!
//! ```
//! # use bishop::*;
//! #
//! fn random_art(data: &[u8]) {
//!     let opts1 = DrawingOptions { top_text: "pass 1".to_string(), ..Default::default() };
//!     let opts2 = DrawingOptions { bottom_text: "pass 2".to_string(), ..Default::default() };
//!
//!     // compute field once
//!     let field = BishopArt::new().chain(data).result();
//!
//!     // then draw it multiple times with different options
//!     println!("{}", field.draw_with_opts(&opts1));
//!     println!("{}", field.draw_with_opts(&opts2));
//! }
//! ```
//!
//! ## For `Read` (file, stdin, etc)
//!
//! ```
//! # use bishop::*;
//! use std::io::{self, Read};
//!
//! fn main() {
//!     // BishopArt implements Write trait
//!     let mut art = BishopArt::new();
//!     io::copy(&mut io::stdin(), &mut art);
//!     println!("{}", art.draw());
//! }
//! ```
//!

/// Module that does the thing
pub mod bishop_art;

mod vec2d;

pub use bishop_art::{BishopArt, BishopResult, DrawingOptions};

/// Module with local errors
pub mod errors {
  use std::{io, result};
  use thiserror as te;

  #[derive(te::Error, Debug)]
  pub enum Error {
    #[error("IO: {0}")]
    IO(#[source] io::Error),

    #[error("Field geometry must be within range: {min_wh:?} - {max_wh:?}")]
    BadGeometry {
      min_wh: (usize, usize),
      max_wh: (usize, usize),
    },
  }

  /// Local result type
  pub type Result<T> = result::Result<T, Error>;
}
