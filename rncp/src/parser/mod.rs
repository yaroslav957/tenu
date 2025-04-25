mod lex;

use crate::error::{Error, ParserError, Result};

pub trait ValueParser<T: Sized> {
    fn parse(&self, arg: &'static str) -> Result<T>;
}

pub struct IntParser;
pub struct StrParser;

impl ValueParser<usize> for IntParser {
    fn parse(&self, arg: &'static str) -> Result<usize> {
        arg.parse()
            .or(Err(Error::ParseError(ParserError::IntParse(arg))))
    }
}
