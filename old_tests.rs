#[cfg(test)]
mod tests {
    use super::*;
    use crate::BishopError;
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
                    Err(BishopError::Err { msg }) if msg.starts_with(err) => (),
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
        let hd = hex::decode(h).unwrap();
        println!("{}\n", art_str(hd.as_slice(), &cfg)?);

        println!("file art_print");
        let f = File::open("test_stuff/foobar_hash")?;
        let mut bf = BufReader::new(f).bytes();
        art_print(& mut bf, &cfg, |s| println!("{}", s))?;
        println!();

        Ok(())
    }

}

