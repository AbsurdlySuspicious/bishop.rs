extern crate hex;

use self::{Mov::*, Pos::*};

pub type CharList = Vec<char>;
pub type CharSE = (char, char);
pub type FieldXY = Vec<Vec<usize>>;
type PosXY = (usize, usize);

#[derive(Clone, Debug)]
pub struct Options {
    chars: CharList,
    chars_se: CharSE,
    field_w: usize,
    field_h: usize,
}

pub const DEFAULT_CHARS: &str = " .o+=*BOX@%&#/^";
pub const DEFAULT_SE: CharSE = ('S', 'E');
pub const DEFAULT_SIZE_WH: PosXY = (17, 9);

impl Options {
    fn mk_chars(s: &str) -> CharList {
        s.chars().collect()
    }

    pub fn new((field_w, field_h): PosXY, chars: &str, chars_se: CharSE) -> Options {
        Options {
            chars: Options::mk_chars(DEFAULT_CHARS),
            chars_se,
            field_w,
            field_h,
        }
    }

    pub fn default() -> Options {
        Options::new(DEFAULT_SIZE_WH, DEFAULT_CHARS, DEFAULT_SE)
    }
}

#[derive(Debug)]
enum Mov {
    MU,  // Up
    MD,  // Down
    MR,  // Right
    ML,  // Left
    MUR, // Up-Right
    MUL, // Up-Left
    MDR, // Down-Right
    MDL, // Down-Left
    Mno, // Stay
}

#[derive(Debug)]
enum Pos {
    PT,  // Top row
    PB,  // Bottom row
    PL,  // Left col
    PR,  // Right col
    PM,  // Middle
    PCa, // UL corner
    PCb, // UR corner
    PCc, // DL corner
    PCd, // DR corner
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

// todo consider using closures for walker-related functions

fn mov_vector(pos: &Pos, a: bool, b: bool) -> Mov {
    // src direction
    let v = if a && b {
        MDR
    } else if a {
        MDL
    } else if b {
        MUR
    } else {
        MUL
    };

    // move rules
    match (pos, v) {
        (PT, MUL) => ML,
        (PT, MUR) => MR,
        (PB, MDL) => ML,
        (PB, MDR) => MR,
        (PL, MUL) => MU,
        (PL, MDL) => MD,
        (PR, MUR) => MU,
        (PR, MDR) => MD,
        (PCa, MUL) => Mno,
        (PCa, MUR) => MR,
        (PCa, MDL) => MD,
        (PCb, MUL) => ML,
        (PCb, MUR) => Mno,
        (PCb, MDR) => MD,
        (PCc, MUL) => MU,
        (PCc, MDL) => Mno,
        (PCc, MDR) => MR,
        (PCd, MUR) => MU,
        (PCd, MDL) => ML,
        (PCd, MDR) => Mno,
        (_, v) => v,
    }
}

fn current_pos((x, y): PosXY, (lx, ly): PosXY) -> Pos {
    assert!(x <= lx && y <= ly);

    let (xr, yb) = (x == lx, y == ly);
    let (xl, yt) = (x == 0, y == 0);

    if !(xl || xr || yt || yb) {
        PM
    } else if xl && yt {
        PCa
    } else if xr && yt {
        PCb
    } else if xl && yb {
        PCc
    } else if xr && yb {
        PCd
    } else if yt {
        PT
    } else if yb {
        PB
    } else if xl {
        PL
    } else if xr {
        PR
    } else {
        panic!("impossible position")
    }
}

fn mov_xy(pos: PosXY, lp: PosXY, a: bool, b: bool) -> PosXY {
    let (x, y) = pos;
    let pt = current_pos(pos, lp);
    let mvv = mov_vector(&pt, a, b);
    let nc = match mvv {
        MUL => (x - 1, y - 1),
        MUR => (x + 1, y - 1),
        MDL => (x - 1, y + 1),
        MDR => (x + 1, y + 1),
        MU => (x, y - 1),
        MD => (x, y + 1),
        ML => (x - 1, y),
        MR => (x + 1, y),
        Mno => (x, y),
    };
    eprintln!(
        "mov_xy pos{:?} lp{:?} pt{:?} mvv{:?} -> nc{:?}",
        pos, lp, pt, mvv, nc
    );
    nc
}

fn walker<I>(bytes: I, cfg: &Options) -> FieldXY
where
    I: Iterator<Item = u8>,
{
    let char_max = cfg.chars.len();
    let (char_s, char_e) = (char_max + 1, char_max + 2);
    let (fw, fh) = (cfg.field_w, cfg.field_h);
    let (lx, ly) = (fw - 1, fh - 1);
    let lp = (lx, ly); // todo rm
    let (sx, sy) = (lx / 2, ly / 2);

    let c_pos = |(x, y): PosXY| -> Pos {
        let (xl, xr, yt, yb) =
            (x == 0, x == lx, y == 0, y == ly);

        if xl {
            if yt { PCa }
            else if yb { PCc }
            else { PL }
        }
        else if xr {
            if yt { PCb }
            else if yb { PCd }
            else { PR }
        }
        else if yt { PT }
        else if yb { PB }
        else { PM }
    };

    let mut map_xy = vec![vec![0usize; fh]; fw];
    let mut pos = (sx, sy);

    map_xy[sx][sy] = char_s;

    for b in bytes {
        for &(a, b) in &bit_pairs(b) {
            pos = mov_xy(pos, lp, a, b);
            let (x, y) = pos;
            let ps = &mut map_xy[x][y];
            if *ps < char_max {
                (*ps) = *ps + 1;
            }
        }
    }

    let (x, y) = pos;
    map_xy[x][y] = char_e;
    map_xy
}

// todo draw

pub fn heh(h: &str) {
    let data = hex::decode(h).unwrap();
    let cfg = Options::default();
    let field = walker(data.into_iter(), &cfg);

    let (fw, fh) = (cfg.field_w, cfg.field_h);
    println!("field:");
    for y in 0..=(fh - 1) {
        let mut line = vec![0usize; fw];
        for x in 0..=(fw - 1) {
            line[x] = field[x][y];
        }
        println!("{:?}", line);
    }
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
