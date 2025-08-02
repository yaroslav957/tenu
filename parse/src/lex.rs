use core::iter::Peekable;
use core::str::Chars;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Option(&'a str, Option<&'a str>),
    Value(&'a str),
    Error(Error<'a>),
}

pub enum ArgType {
    /// No argument expected
    None,
    /// Argument required
    Required,
    /// Optional argument
    Option,
}

///                   long name     type     short name
pub type CliOption = (&'static str, ArgType, char);

/// Since we need to know declared options in a parsing stage,
/// lookup table is declared here
pub struct LookupTable(pub &'static [CliOption]);

/// Using a linear search here because the number of CLI arguments
/// is usually small, so no need to have a hashmap here just for that.
/// GNU's `getopt` lookup table implemented the same way.
impl LookupTable {
    pub fn lookup_short(&self, arg: char) -> Option<&'static CliOption> {
        self.0.iter().find(|row| row.2 == arg)
    }

    pub fn lookup_long(&self, name: &str) -> Option<&'static CliOption> {
        self.0.iter().find(|row| row.0 == name)
    }
}

/// As long as the parser is implemented as a state machine,
/// this enum contains all possible states of parser.
pub enum ParseState<'a> {
    Short { chars: Chars<'a> },
    Long,
    Value,
    None,
}

pub struct Parser<'a, I: Iterator<Item = &'a str>> {
    src: Peekable<I>,
    table: LookupTable,
    state: ParseState<'a>,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    pub fn new(src: I, table: LookupTable) -> Self {
        Self {
            src: src.peekable(),
            table,
            state: ParseState::None,
        }
    }

    fn parse_short(&mut self) -> Option<Token<'a>> {
        if let ParseState::Short { ref mut chars } = self.state {
            if let Some(c) = chars.next() {
                if let Some((name, ty, _)) = self.table.lookup_short(c) {
                    match ty {
                        ArgType::None => {
                            if chars.clone().next().is_none() {
                                // Reset the state if flag is exhausted
                                self.state = ParseState::None;
                            }

                            return Some(Token::Option(name, None));
                        }
                        ArgType::Required | ArgType::Option => {
                            let val = if chars.clone().next().is_some() {
                                let s = chars.as_str();
                                self.state = ParseState::None;
                                Some(s)
                            } else {
                                self.state = ParseState::None;
                                self.src.next()
                            };
                            return Some(Token::Option(name, val));
                        }
                    }
                } else {
                    return Some(Token::Error(Error::UnknownShortOption(c)));
                }
            }
        }
        None
    }

    fn parse_long(&mut self) -> Option<Token<'a>> {
        let raw = self.src.next()?.strip_prefix("--")?;
        let (key, val) = match raw.split_once('=') {
            Some((key, val)) => (key, Some(val)),
            None => (raw, None),
        };

        if let Some((_, ty, _)) = self.table.lookup_long(key) {
            match ty {
                ArgType::Required if val.is_none() => {
                    return Some(Token::Error(Error::MissingArgValue));
                }
                _ => {}
            }
        }

        Some(Token::Option(key, val))
    }

    fn parse_value(&mut self) -> Option<Token<'a>> {
        Some(Token::Value(self.src.next()?))
    }
}

impl<'a, I> Iterator for Parser<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ParseState::None => {
                match self.src.peek()? {
                    s if s.starts_with("--") => {
                        self.state = ParseState::Long;
                    }
                    s if s.starts_with('-') => {
                        let s = self.src.next()?;
                        let chars = s[1..].chars();
                        self.state = ParseState::Short { chars };
                    }
                    _ => {
                        self.state = ParseState::Value;
                    }
                }

                self.next()
            }
            ParseState::Short { .. } => self.parse_short(),
            ParseState::Long => {
                self.state = ParseState::None;
                self.parse_long()
            }
            ParseState::Value => {
                self.state = ParseState::None;
                self.parse_value()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::lex::{ArgType, LookupTable, Parser, Token};

    #[test]
    fn grouped_short_with_value() {
        let args = ["./bin", "-hv", "-oout.jpg", "file1", "file2"]
            .iter()
            .copied();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
                ("output", ArgType::Required, 'o'),
            ]),
        );

        let tokens: Vec<_> = parser.collect();
        assert_eq!(
            tokens,
            Vec::from([
                Token::Value("./bin"),
                Token::Option("help", None),
                Token::Option("verbose", None),
                Token::Option("output", Some("out.jpg")),
                Token::Value("file1"),
                Token::Value("file2"),
            ])
        );
    }

    #[test]
    fn example_input() {
        let args = [
            "./aber",
            "-hv",
            "-ohedgeberry-s-mother.jpg",
            "-v",
            "log.txt",
            "l420.txt",
        ]
        .into_iter();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(
            Vec::from([
                Token::Value("./aber"),
                Token::Option("help", None),
                Token::Option("verbose", None),
                Token::Option("output", Some("hedgeberry-s-mother.jpg")),
                Token::Option("verbose", None),
                Token::Value("log.txt"),
                Token::Value("l420.txt"),
            ]),
            parser.collect::<Vec<_>>()
        )
    }

    #[test]
    fn mixed() {
        let args = ["test", "-ohedgeberry-s-mother.jpg", "-v", "log.txt"].into_iter();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(
            Vec::from([
                Token::Value("test"),
                Token::Option("output", Some("hedgeberry-s-mother.jpg")),
                Token::Option("verbose", None),
                Token::Value("log.txt"),
            ]),
            parser.collect::<Vec<_>>()
        )
    }

    #[test]
    fn short_option_value_mixed() {
        let args = ["-v", "log.txt"].iter().copied();
        let mut parser = Parser::new(args, LookupTable(&[("verbose", ArgType::None, 'v')]));

        assert_eq!(parser.next(), Some(Token::Option("verbose", None)));
        assert_eq!(parser.next(), Some(Token::Value("log.txt")));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn short_option_dummy() {
        let args = ["-o-h"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(parser.next(), Some(Token::Option("output", Some("-h"))));
    }

    #[test]
    fn short_with_required_value() {
        let args = ["-o", "hedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(
            parser.next(),
            Some(Token::Option("output", Some("hedgeberry-s-mother.jpg")))
        );
    }

    #[test]
    fn short_with_required_value_squeezed() {
        let args = ["-ohedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(
            parser.next(),
            Some(Token::Option("output", Some("hedgeberry-s-mother.jpg")))
        );
    }

    #[test]
    fn short_squeezed() {
        let args = ["-hlv"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("help", ArgType::None, 'h'),
                ("list", ArgType::None, 'l'),
                ("version", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(parser.next(), Some(Token::Option("help", None)));
        assert_eq!(parser.next(), Some(Token::Option("list", None)));
        assert_eq!(parser.next(), Some(Token::Option("version", None)));
    }

    #[test]
    fn short_without_value() {
        let args = ["-h"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::None, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_optional_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Option, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_with_optional_value_missing() {
        let args = ["--hello"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Option, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_required_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Required, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_flag_without_value() {
        let args = ["--hello", "world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::None, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
        assert_eq!(parser.next(), Some(Token::Value("world")));
    }

    #[test]
    fn dumb_test() {
        let args = ["hello", "world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[]));
        assert_eq!(parser.next(), Some(Token::Value("hello")));
        assert_eq!(parser.next(), Some(Token::Value("world")));
    }
}
