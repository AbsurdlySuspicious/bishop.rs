use crate::vec2d::*;
use crate::input::*;
use crate::{_raise, Result};

use unicode_width::*;

pub type CharList = Vec<char>;
pub type FieldXY = Vec2D<usize>;

type PosXY = (usize, usize);

/// Fingerprint generation options
#[derive(Clone, Debug)]
pub struct Options {
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

    /// Field width
    pub field_w: usize,

    /// Field height
    pub field_h: usize,

    /// Text for top frame border
    pub top_str: String,

    /// Text for bottom frame border
    pub bot_str: String,
}

/// Maximum size for a field (x, y)
pub const GEOMETRY_LIMITS_MAX: PosXY = (500, 500);

/// Minimum size for a field (x, y)
pub const GEOMETRY_LIMITS_MIN: PosXY = (5, 5);

/// Default field size (x, y)
pub const DEFAULT_SIZE_WH: PosXY = (17, 9);

/// Default char list (see `Options`)
pub const DEFAULT_CHARS: &str = " .o+=*BOX@%&#/^SE";

/// Default text for frame borders
pub const DEFAULT_TEXT: &str = "";

impl Options {
    pub fn chars_from_str(s: &str) -> CharList {
        s.chars().collect()
    }

    pub fn new((w, h): PosXY, chars: &str, top: &str, bot: &str) -> Result<Options> {
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

        Ok(Options {
            chars: chars.chars().collect(),
            field_w: w,
            field_h: h,
            top_str: top.to_string(),
            bot_str: bot.to_string(),
        })
    }

    pub fn default() -> Options {
        match Options::new(
            DEFAULT_SIZE_WH,
            DEFAULT_CHARS,
            DEFAULT_TEXT,
            DEFAULT_TEXT
        ) {
            Ok(o) => o,
            Err(e) => panic!("Wrong default options: {}", e),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options::default()
    }
}

#[inline]
fn u_add(a: usize, b: isize) -> usize {
    let ub = b.abs() as usize;
    if b < 0 { a - ub } else { a + ub }
}

#[inline]
fn _u_add_alt(a: usize, b: isize) -> usize {
    (a as isize + b) as usize
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
    let mut a = [(false, false); 4];

    for c in 0..=3 {
        let b = 7 - (c * 2) as u8;
        a[c] = (bs(b - 1), bs(b));
    }

    a
}


/// Returns a field with raw values generated from `input`
///
/// # Errors
/// Returns BishopError::Io if any IO error occurred
pub fn walker<I: Input>(bytes: &mut I, cfg: &Options) -> Result<FieldXY> {
    let char_max = cfg.chars.len() - 3; // last char index
    let (char_s, char_e) = (char_max + 1, char_max + 2);
    let (fw, fh) = (cfg.field_w, cfg.field_h);
    let (lx, ly) = (fw - 1, fh - 1);
    let (sx, sy) = (lx / 2, ly / 2);

    let mov_xy = |(x, y): PosXY, a: bool, b: bool| -> PosXY {
        let (xs, ys, xe, ye) = (x == 0, y == 0, x == lx, y == ly);
        let (mut xv, mut yv) = (bit_v(b), bit_v(a));
        if ys && yv < 0 { yv = 0 };
        if xs && xv < 0 { xv = 0 };
        if ye && yv > 0 { yv = 0 };
        if xe && xv > 0 { xv = 0 };
        (u_add(x, xv), u_add(y, yv))
    };

    let mut map_xy = Vec2D::new(fw, fh, 0usize);
    let mut pos = (sx, sy);

    map_xy[pos] = char_s;

    loop {
        let bit = match bytes.read_b()? {
            Some(b) => b,
            None => break
        };

        for &(a, b) in &bit_pairs(bit) {
            pos = mov_xy(pos, a, b);
            let ps = &mut map_xy[pos];
            if *ps < char_max {
                (*ps) = *ps + 1;
            }
        }
    }

    map_xy[pos] = char_e;
    Ok(map_xy)
}

/// Takes raw field from `walker`
/// and then draws a fingerprint from it line-by-line,
/// passing each line to `print` closure
pub fn draw<P>(f: &FieldXY, cfg: &Options, mut print: P)
where
    P: FnMut(&String),
{
    let (w, h, chars) = (cfg.field_w, cfg.field_h, &cfg.chars);

    let fill_dash = |s: &mut String, c: usize| for _ in 0..c { s.push('-') };

    let text_frame = |frame: &mut String, text: &str| {
        let (text_idx, text_ln) = {
            let real_w = w - 2;
            let mut size = 0usize;
            let mut last = 0usize;

            for (i, c) in text.char_indices() {
                let sz = size + c.width().unwrap_or(0);
                if sz < real_w {
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
        fill_dash(frame, dash);
        frame.push('[');
        frame.push_str(&text[..text_idx]);
        frame.push(']');
        fill_dash(frame, dash + pad);
    };

    let make_frame = |text: &str| {
        let mut frame = String::with_capacity(w + 2);
        frame.push('+');

        if text.is_empty() {
            fill_dash(&mut frame, w)
        } else {
            text_frame(&mut frame, text)
        }

        frame.push('+');
        frame
    };

    let (top, bot) = (&cfg.top_str, &cfg.bot_str);

    let top_frame = make_frame(top);
    print(&top_frame);

    for y in 0..h {
        let mut line = String::with_capacity(w + 2);
        line.push('|');
        for x in 0..w {
            let p = f.get(x, y);
            line.push(chars[*p]);
        }
        line.push('|');
        print(&line);
    }

    if bot.is_empty() && top.is_empty() {
        print(&top_frame)
    } else {
        print(&make_frame(bot))
    }
}

/// Takes `input` and prints a fingerprint generated
/// from `input` line-by-line,
/// passing each line to `print` closure
///
/// `input` - any type that implements input::Input.
/// Default implementations available for `&Vec<u8>`,
/// `&[u8]` and `&mut Bytes<Read>`
///
/// # Errors
/// Returns BishopError::Io if any IO error occurred
pub fn art_print<I, F>(input: I, cfg: &Options, print: F) -> Result<()>
where
    I: AsInput,
    F: FnMut(&String),
{
    Ok(draw(&walker(&mut input.bs_input(), cfg)?, cfg, print))
}

/// Takes `input` and returns String with fingerprint image
/// generated from it.
///
/// `input` - any type that implements input::Input.
/// Default implementations available for `&Vec<u8>`,
/// `&[u8]` and `&mut Bytes<Read>`
///
/// # Errors
/// Returns BishopError::Io if any IO error occurred
pub fn art_str<I: AsInput>(input: I, cfg: &Options) -> Result<String> {
    let cap = (cfg.field_w + 3) * (cfg.field_h + 2);
    let mut out = String::with_capacity(cap);

    let p = |s: &String| {
        out.push_str(s);
        out.push('\n');
    };

    art_print(input, cfg, p)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BsError;
    use std::collections::HashMap;
    use std::{fs::File, io::{BufReader, Read}};

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
        ("fc94b0c1e5b0987c5843997697ee9fb7", "\
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
+-----------------+\n"),
        ("731ee54c82233359e3d5e9f6ccf87e1f", "\
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
+-----------------+\n"),
    ];

    #[test]
    fn test_draw_ref() {
        let cfg = Options::default();

        for (hash, art) in REF_ARTS {
            let b = hex::decode(hash).unwrap();

            let mut out = String::new();
            let print = |s: &String| {
                out.push_str(s);
                out.push('\n');
            };

            art_print(b.as_slice(), &cfg, print).unwrap();

            assert_eq!(out, *art);
        }
    }

    #[test]
    fn test_walker_ref() {
        for (hash, art) in REF_ARTS {
            let cfg = Options::default();
            let mut chars = HashMap::new();

            for (i, c) in DEFAULT_CHARS.chars().enumerate() {
                chars.insert(c, i);
            }

            let ref_f: Vec<_> = art.lines()
                .filter(|l| !l.starts_with('+'))
                .map(|l| l.trim_matches('|'))
                .map(|l| l.chars().map(|c| chars[&c]).collect::<Vec<_>>())
                .collect();

            let data = hex::decode(hash).unwrap();
            let r = walker(&mut data.bs_input(), &cfg).unwrap();

            println!("ref: {}, r: {}", ref_f.len(), r.0.len());
            assert_eq!(Vec2D(ref_f), r);
        }
    }

    #[test]
    fn test_cfg_validation() {
        let set = [
            (&[
                Options::new((std::usize::MAX, 15), DEFAULT_CHARS, "", ""),
                Options::new((15, std::usize::MAX), DEFAULT_CHARS, "", ""),
                Options::new((0, 15), DEFAULT_CHARS, "", ""),
                Options::new((15, 0), DEFAULT_CHARS, "", "")
            ][..], "Field geometry"),
            (&[
                Options::new(DEFAULT_SIZE_WH, ".o", "", "")
            ][..], "Char list")
        ];

        for (cs, err) in &set {
            for c in *cs {
                match c {
                    Err(BsError::Err { msg }) if msg.starts_with(err) => (),
                    _ => panic!()
                }
            }
        }
    }

    #[test]
    fn test_inputs() -> Result<()> {
        let cfg = Options::default();

        println!("hex art_str");
        let h = "aec070645fe53ee3b3763059376134f058cc337247c978add178b6ccdfb0019f";
        let hd = hex::decode(h)?;
        println!("{}\n", art_str(hd.as_slice(), &cfg)?);

        println!("file art_print");
        let f = File::open("test_stuff/foobar_hash")?;
        let mut bf = BufReader::new(f).bytes();
        art_print(& mut bf, &cfg, |s| println!("{}", s))?;
        println!();

        Ok(())
    }

}
