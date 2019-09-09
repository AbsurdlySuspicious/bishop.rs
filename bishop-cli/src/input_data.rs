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
    read: usize
}

impl<R: Read> HexInput<R> {
    pub fn new(b: R) -> Self {
        assert_eq!(HEX_BUF_SIZE % 2, 0);
        HexInput { b, buf: vec![], len: 0, read: 0 }
    }
}

impl<R: Read> Input for HexInput<R> {
    fn read_b(&mut self) -> Result<Option<u8>> {
        let buf = &mut self.buf;

        if self.read >= self.len {
            let mut h_buf = [0u8; HEX_BUF_SIZE];

            let mut h_len = loop {
                match self.b.read(&mut h_buf) {
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e.into()),
                    Ok(0) => return Ok(None),
                    Ok(x) => break x
                };
            };

            // eliminate dangling line feeds
            let raw_ln = h_len;
            for i in 1..=raw_ln {
                let l = h_buf[raw_ln - i];
                if l == b'\n' || l == b'\r' {
                    h_len -= 1;
                } else {
                    break;
                }
            }

            // exit if buffer was full of line feeds
            if h_len == 0 {
                return Ok(None)
            }

            let () = match hex::decode(&h_buf[..h_len]) {
                Err(e) => _raise(format!("Hex parser error: {}", e))?,
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


