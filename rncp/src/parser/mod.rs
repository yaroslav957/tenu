use crate::error::Result;

pub trait ValueParser<T: Sized> {
    fn parse(&self, arg: &'static str) -> Result<T>;
}

pub struct IntParser;
pub struct StrParser;

impl ValueParser<usize> for IntParser {
    fn parse(&self, arg: &'static str) -> Result<usize> {
        Ok(arg.parse().unwrap()) // `?`
    }
}
