use crate::Result;
use std::io::{Bytes, Read};

pub trait AsInput {
    type I: Input;
    fn bs_input(self) -> Self::I;
}

impl<'a, B: Read> AsInput for &'a mut Bytes<B> {
    type I = BytesInput<'a, B>;

    fn bs_input(self) -> Self::I {
        BytesInput::new(self)
    }
}

impl<'a> AsInput for &'a [u8] {
    type I = SliceInput<'a>;

    fn bs_input(self) -> Self::I {
        SliceInput::new(self)
    }
}

impl<'a> AsInput for &'a Vec<u8> {
    type I = SliceInput<'a>;

    fn bs_input(self) -> Self::I {
        SliceInput::new(self.as_slice())
    }
}

pub type InputReadResult = Result<Option<u8>>;

/// Input acceptable by `walker` and `art_*` functions
pub trait Input {
    /// Returns next byte from input,
    /// `None` on End of Input or `BsError` on error
    fn read_b(&mut self) -> InputReadResult;
}

pub struct SliceInput<'a> {
    pub input: &'a [u8],
    c: usize
}

impl<'a> SliceInput<'a> {
    pub fn new(s: &[u8]) -> SliceInput {
        SliceInput { input: s, c: 0 }
    }
}

impl<'a> Input for SliceInput<'a> {
    fn read_b(&mut self) -> InputReadResult {
        let i = self.input;
        let res;
        if self.c < i.len() {
            res = Some(i[self.c]);
            self.c += 1;
        } else {
            res = None
        }
        Ok(res)
    }
}

pub struct BytesInput<'a, T: Read> {
    pub input: &'a mut Bytes<T>
}

impl<'a, T: Read> BytesInput<'a, T> {
    pub fn new(b: &'a mut Bytes<T>) -> BytesInput<'a, T> {
        BytesInput { input: b }
    }
}

impl<'a, T: Read> Input for BytesInput<'a, T> {
    fn read_b(&mut self) -> InputReadResult {
        match self.input.next() {
            None => Ok(None),
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(e.into())
        }
    }
}

