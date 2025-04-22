use core::str::Chars;
use alloc::vec::Vec;

pub enum Token<'a> {
    Key(&'a str),
    Value(&'a str),
    Raw(&'a str)
}

pub struct Parser<'a> {
    src: Chars<'a>,

    /// Whether a `--` terminator occured
    is_raw: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            is_raw: false,
            src: src.chars()
        }
    }

    pub fn parse() -> Vec<Token<'a>> {
        todo!()
    }
}

