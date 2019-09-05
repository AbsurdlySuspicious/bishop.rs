use crate::BsError;
use std::io::{Bytes, Read};

/// Type which can be converted to `BsInput`.
/// Used as workaround instead of `Into<T>`
/// because of Rust limitations.
pub trait AsBsInput {
    type I: BsInput;
    fn bs_input(self) -> Self::I;
}

impl<'a, B: Read> AsBsInput for &'a mut Bytes<B> {
    type I = BytesInput<'a, B>;

    fn bs_input(self) -> Self::I {
        BytesInput::new(self)
    }
}

impl<'a> AsBsInput for &'a [u8] {
    type I = SliceInput<'a>;

    fn bs_input(self) -> Self::I {
        SliceInput::new(self)
    }
}

pub type BsReadResult = Result<Option<u8>, BsError>;

/// Input acceptable by `walker` and `art_*` functions
pub trait BsInput {
    /// Returns next byte from input,
    /// `None` on End of Input or `BsError` on error
    fn read_b(&mut self) -> BsReadResult;
}

pub struct SliceInput<'a> {
    pub input: &'a [u8],
    len: usize,
    c: usize
}

impl<'a> SliceInput<'a> {
    pub fn new(s: &[u8]) -> SliceInput {
        SliceInput { input: s, len: s.len(), c: 0 }
    }
}

impl<'a> BsInput for SliceInput<'a> {
    fn read_b(&mut self) -> BsReadResult {
        let res;
        if self.c < self.len {
            res = Some(self.input[self.c]);
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

impl<'a, T: Read> BsInput for BytesInput<'a, T> {
    fn read_b(&mut self) -> BsReadResult {
        match self.input.next() {
            None => Ok(None),
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(e.into())
        }
    }
}

