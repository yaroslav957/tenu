use alloc::vec::Vec;
use core::iter;

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

///                                      (Optional)              
///               long name     type     short name
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

// pub struct Parser<'a, T: AsRef<str>> {
//    src: &'a [T],
//    table: LookupTable,
//    is_raw: bool,
// }
pub struct Parser<'a> {
    src: &'a [&'a str],
    table: LookupTable,
    is_raw: bool,
}

impl<'a> Parser<'a> {
    //         src: &'a [T]
    pub fn new(src: &'a [&'a str], table: LookupTable) -> Self {
        Self {
            src,
            table,
            is_raw: false,
        }
    }

    // и тут уже делаешь с &'a [T]
    pub fn parse(&mut self) -> Result<Vec<Token<'a>>, Error> {
        let mut buffer = Vec::new();
        let mut iter = self.src.iter().peekable();

        while let Some(&arg) = iter.next() {
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

    // TODO: 1. write tests 2. refactoring
    fn parse_arg<I>(
        &self,
        buffer: &mut Vec<Token<'a>>,
        arg: &'a str,
        iter: &mut iter::Peekable<I>,
    ) -> Result<(), Error<'a>>
    where
        I: Iterator<Item = &'a &'a str>,
    {
        let mut chars = arg.char_indices().peekable();

        if let Some((_, '-')) = chars.peek() {
            chars.next();

            if let Some((_, '-')) = chars.peek() {
                chars.next();
                let option_name_start = chars.peek().map(|(i, _)| *i).unwrap(); // TODO

                // Check for '=' for --option=value syntax
                if let Some(eq_pos) = arg[option_name_start..].find('=') {
                    let (name, value) = arg[option_name_start..].split_at(eq_pos);
                    let value = &value[1..]; // skip '='

                    if let Some(_) = self.table.lookup_long(name) {
                        buffer.push(Token::Option(name, Some(value)));
                        return Ok(());
                    } else {
                        return Err(Error::UnknownLongOption(name));
                    }
                }

                let name = &arg[option_name_start..];

                if let Some(opt) = self.table.lookup_long(name) {
                    match opt.1 {
                        ArgType::None => {
                            buffer.push(Token::Option(name, None));
                        }
                        ArgType::Required => {
                            let next = iter.next().copied().unwrap(); // TODO: Missing value
                            buffer.push(Token::Option(name, Some(next)));
                        }
                        ArgType::Option => {
                            let next = iter.peek().copied();
                            if let Some(&next_val) = next {
                                if !next_val.starts_with('-') {
                                    buffer.push(Token::Option(name, Some(next_val)));
                                    iter.next(); // consume value
                                } else {
                                    buffer.push(Token::Option(name, None));
                                }
                            } else {
                                buffer.push(Token::Option(name, None));
                            }
                        }
                    }
                    return Ok(());
                } else {
                    return Err(Error::UnknownLongOption(name));
                }
            } else {
                // Short options like -zov or -o value
                while let Some((opt_start, c)) = chars.next() {
                    if let Some(opt) = self.table.lookup_short(c) {
                        match opt.1 {
                            ArgType::None => {
                                buffer.push(Token::Option(&opt.0, None));
                            }
                            ArgType::Required => {
                                // Check if value is squeezed, e.g., -ofile
                                if let Some((i, _)) = chars.peek().copied() {
                                    let value = &arg[i..];
                                    buffer.push(Token::Option(&opt.0, Some(value)));
                                } else {
                                    let next = iter.next().copied().unwrap(); // TODO
                                    buffer.push(Token::Option(&opt.0, Some(next)));
                                }

                                return Ok(());
                            }
                            ArgType::Option => {
                                if let Some((i, _)) = chars.peek().copied() {
                                    let value = &arg[i..];
                                    buffer.push(Token::Option(&opt.0, Some(value)));
                                    return Ok(());
                                } else {
                                    let next = iter.peek().copied();
                                    if let Some(&next_val) = next {
                                        if !next_val.starts_with('-') {
                                            buffer.push(Token::Option(&opt.0, Some(next_val)));
                                            iter.next();
                                        } else {
                                            buffer.push(Token::Option(&opt.0, None));
                                        }
                                    } else {
                                        buffer.push(Token::Option(&opt.0, None));
                                    }
                                }
                                return Ok(());
                            }
                        }
                    } else {
                        return Err(Error::UnknownShortOption(&arg[opt_start..=opt_start])); // cringy
                    }
                }
                return Ok(());
            }
        }

        buffer.push(Token::Value(arg));

        Ok(())
    }
}
