use alloc::vec::Vec;
use core::iter::{self, Peekable};

use crate::error::Error;

#[derive(Debug)]
pub enum Token<'a> {
    Option(&'a str, Option<&'a str>),
    Value(&'a str),
}

pub enum ArgType {
    /// No argument expected
    None,
    /// Argument required
    Required,
    /// Optional argument
    Option,
}

///                                          (Optional)              
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

pub struct Parser<'a, T> {
    src: &'a [T],
    table: LookupTable,
    is_raw: bool,
}

impl<'a, T: AsRef<str>> Parser<'a, T> {
    pub fn new(src: &'a [T], table: LookupTable) -> Self {
        Self {
            src,
            table,
            is_raw: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token<'a>>, Error> {
        let mut buffer = Vec::new();
        let mut iter = self.src
            .iter()
            .map(T::as_ref)
            .peekable();

        while let Some(arg) = iter.next() {
            if !self.is_raw && arg == "--" {
                self.is_raw = true;
                continue;
            }

            if self.is_raw {
                buffer.push(Token::Value(arg));
                continue;
            }

            self.parse_arg(&mut buffer, arg, &mut iter)?;
        }

        Ok(buffer)
    }

    // TODO: 1. write tests
    fn parse_arg<I>(
        &self,
        buffer: &mut Vec<Token<'a>>,
        arg: &'a str,
        iter: &mut Peekable<I>,
    ) -> Result<(), Error<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut chars = arg.char_indices().peekable();

        if let Some((_, '-')) = chars.peek() {
            chars.next();

            if let Some((_, '-')) = chars.peek() {
                chars.next();
                let option_name_start = chars
                    .peek()
                    .map(|(i, _)| *i)
                    .unwrap(); // TODO
                self.parse_long(&arg[option_name_start..], iter, buffer)?;
            } else {
                return self.parse_short(buffer, iter, arg, &mut chars);
            }
        }

        buffer.push(Token::Value(arg));

        Ok(())
    }

    fn parse_long<I>(
        &self,
        arg: &'a str,
        iter: &mut Peekable<I>,
        buffer: &mut Vec<Token<'a>>
    ) -> Result<(), Error<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        if let Some((name, value)) = arg.split_once('=') {
            if let Some(_) = self.table.lookup_long(name) {
                buffer.push(Token::Option(name, Some(value)));
                return Ok(());
            } else {
                return Err(Error::UnknownLongOption(name));
            }
        }

        if let Some(opt) = self.table.lookup_long(arg) {
            buffer.push(Token::Option(
                arg,
                self.parse_opt_arg(&opt.1, iter)?
            ));
            return Ok(());
        } else {
            return Err(Error::UnknownLongOption(arg));
        }
    }

    fn parse_opt_arg<I>(&self, ty: &ArgType, iter: &mut Peekable<I>)
        -> Result<Option<&'a str>, Error<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        match ty {
            ArgType::None => Ok(None),
            ArgType::Required => if let Some(next) = iter.next() {
                Ok(Some(next))
            } else {
                Err(Error::MissingArg)
            },
            ArgType::Option => if let Some(&next) = iter.peek() {
                if !next.starts_with('-') {
                    iter.next(); // consume value
                    Ok(Some(next))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        }
    }

    /// Short options like -zov or -o value
    fn parse_short<I>(
        &self,
        buffer: &mut Vec<Token<'a>>,
        iter: &mut Peekable<I>,
        arg: &'a str,
        chars: &mut Peekable<impl Iterator<Item = (usize, char)>>
    ) -> Result<(), Error<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        while let Some((opt_start, c)) = chars.next() {
            if let Some(opt) = self.table.lookup_short(c) {
                let opt_arg = match opt.1 {
                    ArgType::None => None,
                    ArgType::Required => {
                        // Check if value is squeezed, e.g., -ofile
                        if let Some(&(i, _)) = chars.peek() {
                            Some(&arg[i..])
                        } else {
                            Some(
                                iter.next().ok_or(Error::MissingArg)?
                            )
                        }
                    }
                    ArgType::Option => {
                        if let Some(&(i, _)) = chars.peek() {
                            Some(&arg[i..])
                        } else {
                            if iter.peek().is_some_and(|&val| !val.starts_with('-')) {
                                Some(iter.next().unwrap())
                            } else {
                                None
                            }
                        }
                    }
                };
                buffer.push(Token::Option(&opt.0, opt_arg));
            } else {
                return Err(Error::UnknownShortOption(&arg[opt_start..=opt_start])); // cringy
            }
        }

        Ok(())
    }
}
