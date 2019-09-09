use crate::vec2d::*;
use crate::input::*;
use crate::{_raise, Result};

use unicode_width::*;

pub type CharList = Vec<char>;
pub type FieldXY = Vec2D<isize>;
pub type PosXY = (usize, usize);

/// Maximum size for a field (x, y): (500, 500)
pub const GEOMETRY_LIMITS_MAX: PosXY = (500, 500);

/// Minimum size for a field (x, y): (5, 5)
pub const GEOMETRY_LIMITS_MIN: PosXY = (5, 5);

/// Default field size (x, y): (17, 9)
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
fn u_add(a: usize, b: isize) -> usize {
    let ub = b.abs() as usize;
    if b < 0 { a - ub } else { a + ub }
}

#[inline]
fn bit_v(b: bool) -> isize {
    if b { 1 } else { -1 }
}

#[inline]
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

/// Options for drawing method
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
            bottom_text: DEFAULT_TEXT.into()
        }
    }
}

/// Resulting field from [`BishopArt`]
///
/// [`BishopArt`]: ./struct.BishopArt.html
pub struct BishopResult {
    field: FieldXY,
    size: PosXY
}

/// Visualizer
#[derive(Debug)]
pub struct BishopArt {
    field_w: usize,
    field_h: usize,
    map: FieldXY,
    pos: PosXY
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
        if chars.len() < 4 {
            return _raise("Char list must be 4 chars or longer");
        }

        let ((min_w, min_h), (max_w, max_h)) =
            (GEOMETRY_LIMITS_MIN, GEOMETRY_LIMITS_MAX);

        if w > max_w || h > max_h || w < min_w || h < min_h {
            let e = format!(
                "Field geometry must be within range: [{},{}] - [{},{}]",
                min_w, min_h, max_w, max_h
            );
            return _raise(e);
        }

        let pos = ((w - 1) / 2, (h - 1) / 2);
        let mut map = Vec2D::new(w, h, 0isize);

        map[pos] = VALUE_S;

        Ok(BishopArt {
            field_w: w,
            field_h: h,
            map, pos
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
        for byte in i {
            for &(a, b) in &bit_pairs(byte) {
                let pos = mov(self.pos, a, b);
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
    /// use bishop::bishop2::BishopArt;
    /// let art = BishopArt::new().chain(b"foo").chain(b"bar").draw();
    /// ```
    ///
    /// [`input()`]: ./struct.BishopArt.html#method.input
    pub fn chain<T: AsRef<[u8]>>(&mut self, i: T) -> &mut Self {
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
            size: (self.field_w, self.field_h)
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

    /// Draw resulting field to String using
    /// parameters from [`DrawingOptions`]
    ///
    /// [`DrawingOptions`]: ./struct.DrawingOptions.html
    pub fn draw_with_opts(&self, o: &DrawingOptions) -> String {
        unimplemented!()
    }

    /// Draw resulting field to String using default parameters
    pub fn draw(&self) -> String {
        self.draw_with_opts(&DrawingOptions::default())
    }

}
