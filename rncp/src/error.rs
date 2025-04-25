use core::{error, fmt, result, str::Utf8Error};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidValue,
    InvalidUtf8Arg,
    MissingArg,
    MissingArgValue,
    DuplicatedArg,
    ParseError(ParserError),
}

/// 'static, cuz arg has 'static lifetime
#[derive(Debug)]
pub enum ParserError {
    IntParse(&'static str),
    StrParse(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue => write!(f, "Invalid argument value"),
            Self::InvalidUtf8Arg => write!(f, "Invalid argument format (Non UTF-8)"),
            Self::MissingArg => write!(f, "Missing argument"),
            Self::MissingArgValue => write!(f, "Missing option value"),
            Self::DuplicatedArg => write!(f, "Duplicated argument"),
            Self::ParseError(e) => write!(f, "{e}"),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntParse(arg) => write!(f, "Error while parsing {arg} as integer"),
            Self::StrParse(arg) => write!(f, "Error while parsing {arg} as string"),
        }
    }
}

impl From<ParserError> for Error {
    fn from(e: ParserError) -> Self {
        Error::ParseError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::InvalidUtf8Arg
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
