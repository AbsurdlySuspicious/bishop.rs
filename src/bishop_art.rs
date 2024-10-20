use crate::errors::{Error, Result};
use crate::vec2d::*;

use std::io::{self, Write};
use unicode_width::*;

pub type CharList = Vec<char>;
pub type FieldXY = Vec2D<isize>;
pub type PosXY = (usize, usize);

/// Maximum size for a field (x, y): `(500, 500)`
pub const GEOMETRY_LIMITS_MAX: PosXY = (500, 500);

/// Minimum size for a field (x, y): `(5, 5)`
pub const GEOMETRY_LIMITS_MIN: PosXY = (5, 5);

/// Default field size (x, y): `(17, 9)`
pub const DEFAULT_SIZE_WH: PosXY = (17, 9);

/// Default char list (see [`DrawingOptions`])
///
/// [`DrawingOptions`]: ./struct.DrawingOptions.html
pub const DEFAULT_CHARS: &str = " .o+=*BOX@%&#/^SE";

/// Default text for frame borders
pub const DEFAULT_TEXT: &str = "";

const VALUE_MAX: isize = std::isize::MAX;
const VALUE_S: isize = -1;
const VALUE_E: isize = -2;

#[inline]
#[rustfmt::skip]
fn u_add(a: usize, b: isize) -> usize {
  let ub = b.abs() as usize;
  if b < 0 { a - ub } else { a + ub }
}

#[inline]
#[rustfmt::skip]
fn bit_v(b: bool) -> isize {
  if b { 1 } else { -1 }
}

#[inline]
#[rustfmt::skip]
fn bit_set_le(byte: u8, bit: u8) -> bool {
  assert!(bit <= 7);
  ((byte >> (7 - bit)) & 1) == 1
}

fn bit_pairs(byte: u8) -> [(bool, bool); 4] {
  let bs = |b| bit_set_le(byte, b);
  let mut pairs = [(false, false); 4];

  for c in 0..=3 {
    let b = 7 - (c * 2) as u8;
    pairs[c] = (bs(b - 1), bs(b));
  }

  pairs
}

/// Options for drawing methods
pub struct DrawingOptions {
  /// Vector of chars used for fingerprint
  ///
  /// Each char is treated as:
  ///
  /// Index  | Description             | Default          |
  /// -------|-------------------------|------------------|
  /// `0`    | Field background        | ` `              |
  /// `1..n` | Chars used for drawing  | `.o+=*BOX@%&#/^` |
  /// `n+1`  | Char for start position | `S`              |
  /// `n+2`  | Char for last position  | `E`              |
  ///
  /// Each non-background char indicates how many
  /// times bishop has been on this position.
  ///
  /// Start and end chars overwrites the real value.
  ///
  /// Char list must be at least 4 chars long,
  /// but secure char list is at least 18 chars long
  /// and only consists of clearly distinguishable symbols.
  pub chars: CharList,

  /// Text for top frame border
  pub top_text: String,

  /// Text for bottom frame border
  pub bottom_text: String,
}

impl DrawingOptions {
  /// Returns DrawingOptions with default parameters
  pub fn default() -> DrawingOptions {
    DrawingOptions {
      chars: DEFAULT_CHARS.chars().collect(),
      top_text: DEFAULT_TEXT.into(),
      bottom_text: DEFAULT_TEXT.into(),
    }
  }
}

impl Default for DrawingOptions {
  fn default() -> Self {
    DrawingOptions::default()
  }
}

/// Resulting field from [`BishopArt`]
///
/// [`BishopArt`]: ./struct.BishopArt.html
pub struct BishopResult {
  field: FieldXY,
  size: PosXY,
}

/// Visualizer
#[derive(Debug)]
pub struct BishopArt {
  field_w: usize,
  field_h: usize,
  map: FieldXY,
  pos: PosXY,
}

impl BishopArt {
  /// Creates new BishopArt instance
  ///
  /// # Arguments
  /// + `w` - width of the field
  /// + `h` - height of the field
  ///
  /// Field size must be within limits defined in
  /// [`GEOMETRY_LIMITS_MIN`] and [`GEOMETRY_LIMITS_MAX`]
  ///
  /// # Errors
  ///
  /// Returns [`BishopError::Err`] on limits violation
  ///
  /// [`BishopError::Err`]: ../enum.BishopError.html
  /// [`GEOMETRY_LIMITS_MIN`]: ./constant.GEOMETRY_LIMITS_MIN.html
  /// [`GEOMETRY_LIMITS_MAX`]: ./constant.GEOMETRY_LIMITS_MAX.html
  pub fn with_size(w: usize, h: usize) -> Result<BishopArt> {
    let ((min_w, min_h), (max_w, max_h)) = (GEOMETRY_LIMITS_MIN, GEOMETRY_LIMITS_MAX);

    if w > max_w || h > max_h || w < min_w || h < min_h {
      return Err(Error::BadGeometry {
        min_wh: GEOMETRY_LIMITS_MIN,
        max_wh: GEOMETRY_LIMITS_MAX,
      });
    }

    let pos = ((w - 1) / 2, (h - 1) / 2);
    let mut map = Vec2D::new(w, h, 0isize);

    map[pos] = VALUE_S;

    Ok(BishopArt {
      field_w: w,
      field_h: h,
      map,
      pos,
    })
  }

  /// Creates new BishopArt instance with default field size
  pub fn new() -> BishopArt {
    let (w, h) = DEFAULT_SIZE_WH;
    match BishopArt::with_size(w, h) {
      Ok(o) => o,
      Err(e) => panic!("Wrong default: {}", e),
    }
  }

  #[rustfmt::skip]
  fn mov(&self, (x, y): PosXY, a: bool, b: bool) -> PosXY {
    let (lx, ly) = (self.field_w - 1, self.field_h - 1);
    let (xs, ys, xe, ye) = (x == 0, y == 0, x == lx, y == ly);
    let (mut xv, mut yv) = (bit_v(b), bit_v(a));
    if ys && yv < 0 { yv = 0 };
    if xs && xv < 0 { xv = 0 };
    if ye && yv > 0 { yv = 0 };
    if xe && xv > 0 { xv = 0 };
    (u_add(x, xv), u_add(y, yv))
  }

  /// Push bytes for visualising.
  ///
  /// You can push any amount of data until [`result()`] is called
  ///
  /// Note that pushed data is drawn on field as-is,
  /// without any additional hashing.
  /// Amount of data effective for visualizing
  /// on field with default size is somewhere around 64 bytes.
  /// Consider using hashing if your data is bigger than this
  ///
  /// [`result()`]: ./struct.BishopArt.html#method.result
  pub fn input<T: AsRef<[u8]>>(&mut self, i: T) {
    for &byte in i.as_ref() {
      for &(a, b) in &bit_pairs(byte) {
        let pos = self.mov(self.pos, a, b);
        let p = &mut self.map[pos];
        let v = *p;

        self.pos = pos;

        if v >= 0 && v < VALUE_MAX {
          *p = v + 1;
        }
      }
    }
  }

  /// Same as [`input()`] but suitable for chaining
  ///
  /// # Example
  ///
  /// ```rust
  /// # use bishop::*;
  /// let art = BishopArt::new()
  ///     .chain(b"foo")
  ///     .chain(b"bar")
  ///     .draw();
  /// ```
  ///
  /// [`input()`]: ./struct.BishopArt.html#method.input
  pub fn chain<T: AsRef<[u8]>>(mut self, i: T) -> Self {
    self.input(i);
    self
  }

  /// Finalize the field and return [`BishopResult`]
  ///
  /// Note that this method consumes `BishopArt`
  ///
  /// [`BishopResult`]: ./struct.BishopResult.html
  pub fn result(self) -> BishopResult {
    let mut f = self.map;
    f[self.pos] = VALUE_E;

    BishopResult {
      field: f,
      size: (self.field_w, self.field_h),
    }
  }

  /// Draw resulting field to String using
  /// parameters from [`DrawingOptions`]
  ///
  /// Note that this method calls [`result()`]
  /// internally and therefore consumes `BishopArt`.
  /// If you want to reuse generated field for
  /// drawing it multiple times - use [`result()`]
  /// manually.
  ///
  /// [`DrawingOptions`]: ./struct.DrawingOptions.html
  /// [`result()`]: ./struct.BishopArt.html#method.result
  pub fn draw_with_opts(self, o: &DrawingOptions) -> String {
    self.result().draw_with_opts(o)
  }

  /// Draw resulting field to String using default parameters
  ///
  /// Note that this method calls [`result()`]
  /// internally and therefore consumes `BishopArt`.
  /// If you want to reuse generated field for
  /// drawing it multiple times - use [`result()`]
  /// manually.
  ///
  /// [`result()`]: ./struct.BishopArt.html#method.result
  pub fn draw(self) -> String {
    self.result().draw()
  }
}

impl Write for BishopArt {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let ln = buf.len();
    if ln > 0 {
      self.input(buf);
    }

    Ok(ln)
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl BishopResult {
  /// Get width of resulting field
  pub fn width(&self) -> usize {
    self.size.0
  }

  /// Get height of resulting field
  pub fn height(&self) -> usize {
    self.size.1
  }

  /// Get resulting field
  pub fn field(&self) -> &FieldXY {
    &self.field
  }

  fn fill_dash(s: &mut String, c: usize) {
    for _ in 0..c {
      s.push('-')
    }
  }

  /// Draw resulting field to String using
  /// parameters from [`DrawingOptions`]
  ///
  /// # Panics
  ///
  /// This function panics if char list length
  /// is less than 4 or more than isize::MAX
  ///
  /// [`DrawingOptions`]: ./struct.DrawingOptions.html
  pub fn draw_with_opts(&self, o: &DrawingOptions) -> String {
    let (w, h) = self.size;
    let chr: &Vec<char> = &o.chars;
    let chr_ln = chr.len();

    if chr_ln < 4 || chr_ln > (std::isize::MAX as usize) {
      panic!("Char list length must be 4 <= n <= isize::MAX");
    }

    let chr_sub_ln = (chr_ln - 2) as isize; // length w/o SE chars
    let (chr_last, chr_s, chr_e) = match chr[chr_ln - 3..] {
      [l, s, e] => (l, s, e),
      _ => unreachable!(),
    };

    let v_frame = |s: &mut String, text: &str| {
      s.push('+');
      if text.is_empty() {
        Self::fill_dash(s, w)
      } else {
        let (text_idx, text_ln) = {
          let real_w = w - 2;
          let mut size = 0usize;
          let mut last = 0usize;

          for (i, c) in text.char_indices() {
            let sz = size + c.width().unwrap_or(0);
            if sz <= real_w {
              last = i + c.len_utf8();
              size = sz;
            } else {
              break;
            }
          }

          (last, size)
        };

        let fill_w = w - (text_ln + 2);
        let (dash, pad) = (fill_w / 2, fill_w % 2);
        Self::fill_dash(s, dash);
        s.push('[');
        s.push_str(&text[..text_idx]);
        s.push(']');
        Self::fill_dash(s, dash + pad);
      }
      s.push_str("+\n");
    };

    let line = |s: &mut String, y: usize| {
      s.push('|');
      for x in 0..w {
        let c = match *self.field.get(x, y) {
          VALUE_E => chr_e,
          VALUE_S => chr_s,
          v if v < 0 => unreachable!(),
          v if v < chr_sub_ln => chr[v as usize],
          _ => chr_last,
        };
        s.push(c);
      }
      s.push_str("|\n")
    };

    // (width + 2x pipe + \n) * (height + top + bottom)
    let cap = (w + 3) * (h + 2);
    let mut out = String::with_capacity(cap);

    v_frame(&mut out, &o.top_text);

    for y in 0..h {
      line(&mut out, y);
    }

    v_frame(&mut out, &o.bottom_text);

    //eprintln!("draw cap: {}, real cap: {}", cap, out.capacity());
    out
  }

  /// Draw resulting field to String using default parameters
  pub fn draw(&self) -> String {
    self.draw_with_opts(&DrawingOptions::default())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn test_is_bit_set() {
    let set = [
      (0xfc_u8, [1, 1, 1, 1, 1, 1, 0, 0]),
      (0xed_u8, [1, 1, 1, 0, 1, 1, 0, 1]),
    ];

    for (num, bits) in &set {
      for (i, b) in bits.iter().enumerate() {
        assert_eq!(bit_set_le(*num, i as u8), *b == 1);
      }
    }
  }

  #[test]
  fn test_bit_pairs() {
    let n = 0xf4_u8;
    let nb: Vec<_> = [(1, 1), (1, 1), (0, 1), (0, 0)]
      .iter()
      .rev()
      .map(|(a, b)| (*a == 1, *b == 1))
      .collect();

    assert_eq!(bit_pairs(n)[..], nb[..]);
  }

  // reference arts are taken from page 16 of specification
  // http://www.dirk-loss.de/sshvis/drunken_bishop.pdf

  const REF_ARTS: &[(&str, &str)] = &[
    (
      "fc94b0c1e5b0987c5843997697ee9fb7",
      "\
+-----------------+
|       .=o.  .   |
|     . *+*. o    |
|      =.*..o     |
|       o + ..    |
|        S o.     |
|         o  .    |
|          .  . . |
|              o .|
|               E.|
+-----------------+\n",
    ),
    (
      "731ee54c82233359e3d5e9f6ccf87e1f",
      "\
+-----------------+
|        o .. .   |
|       + +  o    |
|      = + ..o    |
|       + . *o    |
|        S o.o=   |
|         + .. +  |
|          .  . E |
|              . o|
|             ...o|
+-----------------+\n",
    ),
  ];

  #[test]
  fn test_draw_ref() {
    let opts = DrawingOptions::default();

    for (hash, art) in REF_ARTS {
      let b = hex::decode(hash).unwrap();

      let out = BishopArt::new().chain(b).draw_with_opts(&opts);

      println!("a_line | r_line");
      for (a_line, r_line) in art.lines().zip(out.lines()) {
        println!("{} {}", a_line, r_line);
      }

      assert_eq!(out, *art);
    }
  }

  #[test]
  fn test_walker_ref() {
    let mut chars = HashMap::new();
    let cl = DEFAULT_CHARS.len();

    for (i, c) in DEFAULT_CHARS.chars().enumerate() {
      let ir = match i {
        i if i == cl - 1 => VALUE_E,
        i if i == cl - 2 => VALUE_S,
        i => i as isize,
      };
      chars.insert(c, ir);
    }

    for (hash, art) in REF_ARTS {
      let ref_f: Vec<_> = art
        .lines()
        .filter(|l| !l.starts_with('+'))
        .map(|l| l.trim_matches('|'))
        .map(|l| l.chars().map(|c| chars[&c]).collect::<Vec<_>>())
        .flatten()
        .collect();

      let data = hex::decode(hash).unwrap();
      let r = BishopArt::new().chain(data).result();

      println!("ref: {}, r: {}", ref_f.len(), r.field.vec.len());
      assert_eq!(ref_f, r.field.vec);
    }
  }
}
