use crate::{cursor::FlatCursor, error::Error};

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Option(&'a str, Option<&'a str>),
    Value(&'a str),
    Error(Error<'a>)
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

pub enum ParseState {
    Short {
        maybe_long: bool
    },
    Long,
    Value,
    None,
}

pub struct Parser<'a, I: Iterator> {
    src: FlatCursor<'a, I>,
    table: LookupTable,
    state: ParseState
}

impl<'a, I> Parser<'a, I>
where 
    I: Iterator<Item = &'a str>
{
    pub fn new(src: I, table: LookupTable) -> Self {
        Self {
            src: FlatCursor::new(src),
            table,
            state: ParseState::None
        }
    }
    
    fn parse_short(&mut self) -> Option<Token<'a>> {
        match self.src.peek() {
            Some('-') if matches!(self.state, ParseState::Short { maybe_long: true }) => {
                self.src.next();
                self.state = ParseState::Long;
                self.next()
            }
            Some(_) => {
                self.state = ParseState::Short { maybe_long: false };
                let opt = self.src.next()?;

                if let Some((name, ty, _)) = self.table.lookup_short(opt) {
                    match ty {
                        ArgType::None => {
                            Some(Token::Option(&name, None))
                        },
                        ArgType::Required | ArgType::Option => {
                            self.state = ParseState::None;
                            // TODO: Handle required value missing
                            match self.src.next_str() {
                                // No value squeezed
                                Some("") => Some(Token::Option(&name, self.src.next_str())),
                                val => Some(Token::Option(&name, val))
                            }
                        }
                    }
                } else {
                    panic!("Unknown short option: {opt}") // Unknown short option
                }

            }
            _ => None
        }
    }
    
    fn parse_long(&mut self) -> Option<Token<'a>> {
        let raw_arg = self.src.next_str().unwrap_or_default();
        let (key, val) = match raw_arg.split_once('=') {
            Some((key, val)) => (key, Some(val)),
            _ => (raw_arg, None)
        };

        if let Some((_, ty, _)) = self.table.lookup_long(key) {
            match ty {
                ArgType::Required => {
                    if val.is_none() {
                        todo!() // Тут надо высрать ошибку
                    }
                }
                _ => {}
            }
        }

        self.state = ParseState::None;

        Some(Token::Option(key, val))
    }
    
    fn parse_value(&mut self) -> Option<Token<'a>> {
        self.state = ParseState::None;
        Some(Token::Value(self.src.next_str().unwrap_or_default()))
    }
}

impl<'a, I> Iterator for Parser<'a, I>
where 
    I: Iterator<Item = &'a str>
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ParseState::None => {
                match self.src.peek() {
                    Some('-') => {
                        self.src.next();
                        self.state = ParseState::Short {
                            maybe_long: true
                        };
                        self.next()
                    }
                    Some(_) => {
                        self.state = ParseState::Value;
                        self.next()
                    }
                    _ => None
                }
            }
            ParseState::Short { .. } => self.parse_short(),
            ParseState::Long => self.parse_long(),
            ParseState::Value => self.parse_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::lex::{ArgType, LookupTable, Parser, Token};

    #[test]
    fn example() {
        let args = [
            "./myprogram",
            "-hv",
            "-ohedgeberry-s-mother.jpg",
            "-v",
            "log.txt",
            "l420.txt",
        ]
        .into_iter();
        let parser = Parser::new(args, LookupTable(&[
            ("output", ArgType::Required, 'o'),
            ("help", ArgType::None, 'h'),
            ("verbose", ArgType::None, 'v')
        ]));
        assert_eq!(
            Vec::from([
                Token::Value("./myprogram"),
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
        let args = [
            "test",
            "-ohedgeberry-s-mother.jpg",
            "-v",
            "log.txt",
        ]
        .into_iter();
        let parser = Parser::new(args, LookupTable(&[
            ("output", ArgType::Required, 'o'),
            ("help", ArgType::None, 'h'),
            ("verbose", ArgType::None, 'v')
        ]));
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
        let mut parser = Parser::new(args, LookupTable(&[
            ("verbose", ArgType::None, 'v')
        ]));

        assert_eq!(parser.next(), Some(Token::Option("verbose", None)));
        assert_eq!(parser.next(), Some(Token::Value("log.txt")));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn short_option_dummy() {
        let args = ["-o-h"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("output", ArgType::Required, 'o'),
            ("help", ArgType::None, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("output", Some("-h"))));
    }

    #[test]
    fn short_with_required_value() {
        let args = ["-o", "hedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("output", ArgType::Required, 'o'),
            ("help", ArgType::None, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("output", Some("hedgeberry-s-mother.jpg"))));
    }

    #[test]
    fn short_with_required_value_squeezed() {
        let args = ["-ohedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("output", ArgType::Required, 'o'),
            ("help", ArgType::None, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("output", Some("hedgeberry-s-mother.jpg"))));
    }

    #[test]
    fn short_squeezed() {
        let args = ["-hlv"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("help",    ArgType::None, 'h'),
            ("list",    ArgType::None, 'l'),
            ("version", ArgType::None, 'v'),
        ]));
        assert_eq!(parser.next(), Some(Token::Option("help", None)));
        assert_eq!(parser.next(), Some(Token::Option("list", None)));
        assert_eq!(parser.next(), Some(Token::Option("version", None)));
    }

    #[test]
    fn short_without_value() {
        let args = ["-h"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("hello", ArgType::None, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_optional_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("hello", ArgType::Option, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_with_optional_value_missing() {
        let args = ["--hello"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("hello", ArgType::Option, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_required_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("hello", ArgType::Required, 'h')
        ]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_flag_without_value() {
        let args = ["--hello", "world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[
            ("hello", ArgType::None, 'h')
        ]));
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
