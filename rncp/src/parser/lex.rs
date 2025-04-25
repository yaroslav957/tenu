use alloc::vec::Vec;
use core::str::Chars;

pub enum Token<'a> {
    Key(&'a str),
    Value(&'a str), // Опшен, ведь может не быть по сути валуе, хз
    Raw(&'a str),
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
            src: src.chars(),
        }
    }

    pub fn parse() -> Vec<Token<'a>> {
        todo!()
    }
}
