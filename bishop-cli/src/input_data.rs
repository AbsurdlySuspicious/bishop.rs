use crate::{_raise_bs, BishopCliError};
use bishop::{input::Input, result::Result};
use std::io::{self, Read, ErrorKind};
use bishop::input::AsInput;

pub struct InputItself<T: Input>(pub T);

impl<T: Input> AsInput for InputItself<T> {
    type I = T;

    fn bs_input(self) -> Self::I {
        self.0
    }
}

fn _raise_io<S: Into<String>, T>(m: S) -> io::Result<T> {
    Err(io::Error::new(ErrorKind::Other, BishopCliError::Other { msg: m.into() }))
}

const HEX_BUF_SIZE: usize = 128;

pub struct HexInput<R: Read> {
    b: R,
    buf: Vec<u8>,
    len: usize,
    read: usize,
    has_lf: bool
}

impl<R: Read> HexInput<R> {
    pub fn new(b: R) -> Self {
        assert_eq!(HEX_BUF_SIZE % 2, 0);
        HexInput { b, buf: vec![], len: 0, read: 0, has_lf: false }
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

impl<R: Read> Input for HexInput<R> {
    fn read_b(&mut self) -> Result<Option<u8>> {
        let buf = &mut self.buf;

        if self.read >= self.len {
            let mut h_buf = [0u8; HEX_BUF_SIZE];

            let h_len = loop {
                let mut h_len = match self.b.read(&mut h_buf) {
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e.into()),
                    Ok(0) => return Ok(None),
                    Ok(x) => x
                };

                // eliminate dangling linefeeds
                let raw_ln = h_len;
                for i in 1..=raw_ln {
                    let l = h_buf[raw_ln - i];
                    if l == b'\n' || l == b'\r' {
                        h_len -= 1;
                    } else {
                        break;
                    }
                }

                if h_len != 0 && self.has_lf {
                    // return error if there's data left after dangling linefeeds
                    return _raise_bs("HexInput: linefeeds in the middle of data")
                } else if raw_ln != h_len {
                    // mark that linefeeds has been encountered
                    self.has_lf = true
                }

                // try again if buffer was full of linefeeds
                if h_len == 0 {
                    continue
                }

                break h_len;
            };

            let () = match hex::decode(&h_buf[..h_len]) {
                Err(e) => _raise_bs(format!("HexInput: {}", e))?,
                Ok(v) => *buf = v
            };

            self.len = buf.len();
            self.read = 0;
        }

        let res = buf[self.read];
        self.read += 1;
        Ok(Some(res))
    }
}


