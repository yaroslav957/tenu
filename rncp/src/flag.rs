use core::ffi::c_char;
use crate::Arg;

pub(crate) enum Flag<'a> {
    /// Represents long `--` suffixed flag
    Long(&'a str),
    /// Represents shor `-` single-char flag, for example `-d`, `-h`.
    ///
    /// The reason it's c_char is because only an ASCII character
    /// can be used as a flag character, so one byte is enough.
    Short(&'a c_char)
}

impl<'a> Flag<'a> {
    pub fn from_arg(arg: Arg<'a>) -> Option<Self> {
        // It's 12:43 AM and i wanna sleep bro
        // bouta rewrite this later
        let mut bytes = arg.0.bytes().peekable();
        match bytes.next() {
            Some(b'-') => match bytes.peek() {
                Some(b'-') => {
                    bytes.next();
                    Some(Self::Long(&arg.0[bytes.len()..]))
                },
                _ => {
                    Some(Self::Short(&arg.0[bytes.len()]))
                }
            }
            _ => None
        }
    }
}

