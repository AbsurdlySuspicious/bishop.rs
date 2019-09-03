use std::fmt::Display;
use std::io::{Bytes, Read};
use std::process::exit;

fn _raise<T, E: Display>(res: Result<T, E>) -> T {
    match res {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

pub type BsReadResult = Result<Option<u8>, String>;

pub trait BsInput<K> {
    fn read_b(&mut self) -> BsReadResult;
}

impl<T> BsInput<T> for T
where
    T: Iterator<Item = u8>,
{
    fn read_b(&mut self) -> BsReadResult {
        Ok(self.next())
    }
}

impl<T: Read> BsInput<T> for Bytes<T>
{
    fn read_b(&mut self) -> BsReadResult {
        match self.next() {
            None => Ok(None),
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(e.to_string())
        }
    }
}

pub struct SliceInput<'a> {
    pub inp: &'a [u8],
    len: usize,
    c: usize
}

impl<'a> SliceInput<'a> {
    pub fn new(s: &[u8]) -> SliceInput {
        SliceInput { inp: s, len: s.len(), c: 0 }
    }
}

impl<'a> BsInput<()> for SliceInput<'a> {
    fn read_b(&mut self) -> Result<Option<u8>, String> {
        let res;
        if self.c < self.len {
            res = Some(self.inp[self.c]);
            self.c += 1;
        } else {
            res = None
        }
        Ok(res)
    }
}
