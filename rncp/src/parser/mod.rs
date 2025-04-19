pub trait ValueParser<T: Sized> {
    fn parse(&self, arg: &'static str) -> T;
}

pub struct IntParser;
pub struct StrParser;

impl ValueParser<usize> for IntParser {
    fn parse(&self, arg: &'static str) -> usize {
        arg.parse().unwrap()
    }
}
