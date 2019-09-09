use bishop::{input::Input, Result, _raise};
use std::io::{Read, ErrorKind};
use bishop::input::AsInput;

pub struct InputItself<T: Input>(pub T);

impl<T: Input> AsInput for InputItself<T> {
    type I = T;

    fn bs_input(self) -> Self::I {
        self.0
    }
}

const HEX_BUF_SIZE: usize = 64;

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
                    return _raise("HexInput: linefeeds in the middle of data")
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
                Err(e) => _raise(format!("HexInput: {}", e))?,
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


