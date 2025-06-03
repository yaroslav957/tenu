use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum Token<'a> {
    Option(&'a str),
    Value(&'a str),
    Raw(&'a str),
}

pub struct Parser<'a> {
    src: &'a [*const u8],
    table: LookupTable,

    /// Whether a `--` terminator occured
    is_raw: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a [*const u8], table: LookupTable) -> Self {
        Self {
            src,
            table,
            is_raw: false,
        }
    }
    /*
    pub fn parse(&mut self) -> Vec<Token<'a>> {
        let mut buffer = Vec::new();

        for raw_arg in self.src {

            let arg = unsafe {
                Box::leak( // SAFETY: just don't worry about this leak)
                    Box::new(Arg::from_ptr(raw_arg.clone()))
                )
            };

            if !self.is_raw && arg.as_str() == Ok("--") {
                self.is_raw = true;
            }

            self.parse_arg(&mut buffer, arg.as_str().unwrap())
        }

        buffer
    }
     */

    fn parse_arg(&self, buffer: &mut Vec<Token<'a>>, arg: &'a str) {
        let mut iter = arg.char_indices().peekable();

        if let Some((_, '-')) = iter.peek().copied() {
            iter.next();

            if let Some((_, '-')) = iter.peek().copied() {
                // looks terrible but no allocations thou
                let Some((start, c)) = iter.next() else {
                    panic!()
                };

                let option_start = start + c.len_utf8();
                let mut end = arg.len();

                for (i, c) in iter {
                    if c.is_alphanumeric() {
                        end = i + c.len_utf8();
                    } else {
                        break;
                    }
                }

                buffer.push(Token::Option(&arg[option_start..end]));
                return;
            } else {
                match iter.peek() {
                    Some((_, c)) => {
                        if let Some(opt) = self.table.lookup_short(*c) {
                            todo!()
                        }
                    }
                    _ => todo!(),
                }
            }

            todo!()
        }

        buffer.push(Token::Value(arg))
    }
}

pub enum ArgType {
    /// No argument expected
    None,
    /// Argument required
    Required,
    /// Optional argument
    Option,
}

///               long name     type     short name
type CliOption = (&'static str, ArgType, char);

/// Since we need to know declared options in a parsing stage,
/// lookup table is declared here
pub struct LookupTable(pub &'static [CliOption]);

impl LookupTable {
    /// Using a linear search here because the number of CLI arguments
    /// is usually small, so no need to have a hashmap here just for that.
    /// GNU's `getopt` lookup table implemented the same way.
    pub fn lookup_short(&self, arg: char) -> Option<&'static CliOption> {
        self.0.iter().find(|row| row.2 == arg)
    }
}
