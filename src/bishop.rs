extern crate hex;

use crate::vec2d::*;

pub type CharList = Vec<char>;
pub type FieldXY = Vec2D<usize>;

type PosXY = (usize, usize);
type BitPair = (bool, bool);

#[derive(Clone, Debug)]
pub struct Options {
    chars: CharList, // [field bg][char]...[start char][end char]
    field_w: usize,
    field_h: usize,
}

pub const DEFAULT_CHARS: &str = " .o+=*BOX@%&#/^SE";
pub const DEFAULT_SIZE_WH: PosXY = (17, 9);

impl Options {
    fn mk_chars(s: &str) -> CharList {
        s.chars().collect()
    }

    pub fn new((field_w, field_h): PosXY, chars: &str) -> Result<Options, &'static str> {
        if chars.len() < 4 {
            Err("Char list must be 4 chars or longer")
        } else {
            Ok(Options {
                chars: Options::mk_chars(chars),
                field_w,
                field_h,
            })
        }
    }

    pub fn default() -> Options {
        match Options::new(DEFAULT_SIZE_WH, DEFAULT_CHARS) {
            Ok(o) => o,
            Err(e) => panic!("Wrong default options: {}", e),
        }
    }
}

#[inline]
fn u_add(a: usize, b: isize) -> usize {
    let ub = b.abs() as usize;
    if b < 0 { a - ub } else { a + ub }
}

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

fn walker<I>(bytes: I, cfg: &Options) -> FieldXY
where
    I: Iterator<Item = u8>,
{
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
    // todo test ^

    let mut map_xy = Vec2D::new(fw, fh, 0usize);
    let mut pos = (sx, sy);

    map_xy[pos] = char_s;

    for b in bytes {
        for &(a, b) in &bit_pairs(b) {
            pos = mov_xy(pos, a, b);
            let ps = &mut map_xy[pos];
            if *ps < char_max {
                (*ps) = *ps + 1;
            }
        }
    }

    map_xy[pos] = char_e;
    map_xy
}

fn draw<P>(f: &FieldXY, cfg: &Options, print: P)
where
    P: Fn(&String),
{
    let (w, h, chars) = (cfg.field_w, cfg.field_h, &cfg.chars);
    let solid_frame = {
        let mut frame = String::with_capacity(w + 2);
        frame.push('+');
        for _ in 0..w {
            frame.push('-');
        }
        frame.push('+');
        frame
    };

    print(&solid_frame);

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

    print(&solid_frame);
}

pub fn heh(h: &str) {
    let data = hex::decode(h).unwrap();
    let cfg = Options::default();
    let field2d = walker(data.into_iter(), &cfg);
    let field = field2d.vec();

    let (fw, fh) = (cfg.field_w, cfg.field_h);
    println!("field:");
    for y in 0..=(fh - 1) {
        let mut line = vec![0usize; fw];
        for x in 0..=(fw - 1) {
            line[x] = field[x][y];
        }
        println!("{:?}", line);
    }

    println!();

    draw(&field2d, &cfg, |s| println!("{}", &s));
}

pub fn heh2() {
    println!("{} {}", true as u8, false as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut nb: Vec<_> = [(1, 1), (1, 1), (0, 1), (0, 0)]
            .iter()
            .rev()
            .map(|(a, b)| (*a == 1, *b == 1))
            .collect();

        assert_eq!(bit_pairs(n)[..], nb[..]);
    }
}
