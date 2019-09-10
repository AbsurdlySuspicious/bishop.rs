use crate::BishopCliError;
use std::io::{self, Read, ErrorKind};

fn _raise_io<S: Into<String>, T>(m: S) -> io::Result<T> {
    Err(io::Error::new(ErrorKind::Other, BishopCliError::Other { msg: m.into() }))
}

const HEX_BUF_SIZE: usize = 128;

pub struct HexInput<R: Read> {
    b: R,
    has_lf: bool
}

impl<R: Read> HexInput<R> {
    pub fn new(b: R) -> Self {
        assert_eq!(HEX_BUF_SIZE % 2, 0);
        HexInput { b, has_lf: false }
    }
}

impl<R: Read> Read for HexInput<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut h_buf = [0u8; HEX_BUF_SIZE];
        let out_len = buf.len();
        let h_len = HEX_BUF_SIZE.min((out_len - (out_len % 2)) * 2);

        let mut rd_len = loop {
            match self.b.read(&mut h_buf[..h_len]) {
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
                Ok(0) => return Ok(0),
                Ok(x) => break x
            };
        };

        if self.has_lf {
            return _raise_io("Data after linefeed");
        }

        // eliminate dangling linefeeds
        let rd_raw = rd_len;
        for i in 1..=rd_raw {
            let l = h_buf[rd_raw - i];
            if l == b'\n' || l == b'\r' {
                rd_len -= 1;
            } else {
                break;
            }
        }

        let rd_delta = rd_raw - rd_len;
        if rd_delta > 2 {
            return _raise_io("More than one dangling linefeed")
        } else if rd_delta != 0 {
            self.has_lf = true;
        }

        if rd_len == 0 {
            return Ok(0)
        }

        /*eprintln!(
            "h_buf: {:?}\nout_len: {}, h_len: {}, rd_raw: {}, rd_len: {}, rd_delta: {}",
            &h_buf[..], out_len, h_len, rd_raw, rd_len, rd_delta
        );*/

        let dec = match hex::decode(&h_buf[..rd_len]) {
            Ok(v) => v,
            Err(e) => return _raise_io(format!("{}", e))
        };

        let dec_ln = dec.len();

        if dec_ln > out_len {
            panic!("small out buf");
        }

        for i in 0..dec_ln {
            buf[i] = dec[i]
        }

        Ok(dec_ln)
    }
}
