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

    // TODO: @timar07 check this && improve if needed
    fn parse_short(&mut self) -> Option<Token<'a>> {
        let (name, ty, is_empty, remaining) = match self.state {
            ParseState::Short { ref mut chars } => {
                let chr = chars.next()?;
                let (name, ty, _) = match self.table.lookup_short(chr) {
                    Some(res) => res,
                    None => return Some(Token::Error(Error::UnknownShortOption(chr))),
                };

                (name, ty, chars.as_str().is_empty(), chars.as_str())
            }
            _ => return None,
        };

        match ty {
            ArgType::None => {
                if is_empty {
                    self.state = ParseState::None;
                }

                Some(Token::Option(name, None))
            }
            _ => {
                let val = if is_empty {
                    self.src.next()
                } else {
                    Some(remaining)
                };

                self.state = ParseState::None;
                Some(Token::Option(name, val))
            }
        }
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
