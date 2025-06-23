#[derive(Debug)]
pub enum LookupKey {
    Unknown,
    Single,
    RequireValue(KeyType),
}

#[derive(Debug)]
pub enum KeyType {
    Int,
    UInt,
    String,
}

#[derive(Debug)]
pub enum LongOrShort<'a> {
    Long(&'a str),
    Short(char),
}

#[derive(Debug)]
pub struct ParseError<'a> {
    pub kind: ParseErrorKind,

    pub key: Option<LongOrShort<'a>>,
    pub value: Option<&'a str>,
    pub expected_ty: Option<KeyType>,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnknownKey,
    ValueRequired,
    UnexpectedValue,
    InvalidKeyType,
    UnexpectedRaw,
}

impl<'a> ParseError<'a> {
    fn unexpected_raw(raw: &'a str) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedRaw,
            key: None,
            value: Some(raw),
            expected_ty: None,
        }
    }

    fn unknown_short_key(key: char) -> Self {
        Self {
            kind: ParseErrorKind::UnknownKey,
            key: Some(LongOrShort::Short(key)),
            value: None,
            expected_ty: None,
        }
    }

    fn unknown_key(key: &'a str) -> Self {
        Self {
            kind: ParseErrorKind::UnknownKey,
            key: Some(LongOrShort::Long(key)),
            value: None,
            expected_ty: None,
        }
    }

    fn unexpected_value(key: &'a str, value: &'a str) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedValue,
            key: Some(LongOrShort::Long(key)),
            value: Some(value),
            expected_ty: None,
        }
    }

    fn expected_value(key: &'a str) -> Self {
        Self {
            kind: ParseErrorKind::ValueRequired,
            key: Some(LongOrShort::Long(key)),
            value: None,
            expected_ty: None,
        }
    }

    fn expected_value_short(key: char) -> Self {
        Self {
            kind: ParseErrorKind::ValueRequired,
            key: Some(LongOrShort::Short(key)),
            value: None,
            expected_ty: None,
        }
    }

    fn invalid_key_type(key: &'a str, value: &'a str, expected_ty: KeyType) -> Self {
        Self {
            kind: ParseErrorKind::InvalidKeyType,
            key: Some(LongOrShort::Long(key)),
            value: Some(value),
            expected_ty: Some(expected_ty),
        }
    }

    fn invalid_short_key_type(key: char, value: &'a str, expected_ty: KeyType) -> Self {
        Self {
            kind: ParseErrorKind::InvalidKeyType,
            key: Some(LongOrShort::Short(key)),
            value: Some(value),
            expected_ty: Some(expected_ty),
        }
    }

    fn invalid_raw_key_type(raw: &'a str, expected_ty: KeyType) -> Self {
        Self {
            kind: ParseErrorKind::InvalidKeyType,
            key: None,
            value: Some(raw),
            expected_ty: Some(expected_ty),
        }
    }
}

pub trait ParseInto<'a> {
    type Into;
    type Error: From<ParseError<'a>>;

    // я слишком сонный, чтобы понять, правильно ли тут расставлены лт, сори
    // допишу утром

    fn lookup_short(&self, short: char) -> LookupKey;
    fn parse_short(&mut self, short: char) -> Result<(), Self::Error>;
    fn parse_short_int(&mut self, short: char, value: i64) -> Result<(), Self::Error>;
    fn parse_short_uint(&mut self, short: char, value: u64) -> Result<(), Self::Error>;
    fn parse_short_str(&mut self, short: char, value: &'a str) -> Result<(), Self::Error>;

    fn lookup_long(&self, long: &str) -> LookupKey;
    fn parse_long(&mut self, long: &str) -> Result<(), Self::Error>;
    fn parse_long_int(&mut self, long: &'a str, value: i64) -> Result<(), Self::Error>;
    fn parse_long_uint(&mut self, long: &'a str, value: u64) -> Result<(), Self::Error>;
    fn parse_long_str(&mut self, long: &'a str, value: &'a str) -> Result<(), Self::Error>;

    fn lookup_raw(&self) -> Option<KeyType>;
    fn parse_raw_str(&mut self, raw: &'a str) -> Result<(), Self::Error>;
    fn parse_raw_int(&mut self, raw: i64) -> Result<(), Self::Error>;
    fn parse_raw_uint(&mut self, raw: u64) -> Result<(), Self::Error>;

    fn parse_end(self) -> Result<Self::Into, Self::Error>;
}

pub trait ParseIntoExt<'a>: ParseInto<'a> + Sized {
    fn parse<I>(self, args: I) -> Result<Self::Into, Self::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        Parser::new(self, args).parse_to_end()
    }
}
impl<'a, T: ParseInto<'a> + Sized> ParseIntoExt<'a> for T {}

pub struct Parser<T, I> {
    raw_args: I,
    parse_only_raw: bool,

    inner_parser: T,
}

impl<T, I> Parser<T, I> {
    pub fn new(parser: T, raw_args: I) -> Self
    where
        T: Sized,
    {
        Self {
            raw_args,
            parse_only_raw: false,

            inner_parser: parser,
        }
    }
}

impl<'a, T: ParseInto<'a>, I: Iterator<Item = &'a str>> Parser<T, I> {
    fn parse_raw(&mut self, arg: &'a str) -> Result<(), T::Error> {
        let Some(info) = self.inner_parser.lookup_raw() else {
            return Err(ParseError::unexpected_raw(arg).into());
        };

        match info {
            KeyType::String => self.inner_parser.parse_raw_str(arg),
            KeyType::Int => arg
                .parse::<i64>()
                .map_err(|_| ParseError::invalid_raw_key_type(arg, info).into())
                .and_then(|value| self.inner_parser.parse_raw_int(value)),
            KeyType::UInt => arg
                .parse::<u64>()
                .map_err(|_| ParseError::invalid_raw_key_type(arg, info).into())
                .and_then(|value| self.inner_parser.parse_raw_uint(value)),
        }
    }

    pub fn parse_once(&mut self) -> Option<Result<(), T::Error>> {
        let arg = self.raw_args.next()?;

        if self.parse_only_raw {
            return Some(self.parse_raw(arg));
        }

        if let Some(arg) = arg.strip_prefix("--") {
            if arg.is_empty() {
                self.parse_only_raw = true;
                return Some(Ok(()));
            }

            let (key, value) = match arg.split_once('=') {
                Some((arg, value)) => (arg, Some(value)),
                None => (arg, None),
            };
            let key_info = self.inner_parser.lookup_long(key);
            match (key_info, value) {
                (LookupKey::Unknown, _) => Some(Err(ParseError::unknown_key(key).into())),
                (LookupKey::Single, Some(value)) => {
                    Some(Err(ParseError::unexpected_value(key, value).into()))
                }
                (LookupKey::Single, None) => Some(self.inner_parser.parse_long(key)),
                (LookupKey::RequireValue(ty), value) => {
                    let value = match value {
                        Some(v) => v,
                        None => {
                            let Some(value) = self.raw_args.next() else {
                                return Some(Err(ParseError::expected_value(key).into()));
                            };
                            value
                        }
                    };

                    match ty {
                        KeyType::String => Some(self.inner_parser.parse_long_str(key, value)),
                        KeyType::Int => Some(
                            value
                                .parse::<i64>()
                                .map_err(|_| ParseError::invalid_key_type(key, value, ty).into())
                                .and_then(|value| self.inner_parser.parse_long_int(key, value)),
                        ),
                        KeyType::UInt => Some(
                            value
                                .parse::<u64>()
                                .map_err(|_| ParseError::invalid_key_type(key, value, ty).into())
                                .and_then(|value| self.inner_parser.parse_long_uint(key, value)),
                        ),
                    }
                }
            }
        } else if let Some(mut arg) = arg
            .strip_prefix("-")
            .filter(|v| !v.is_empty())
            .map(|v| v.chars())
        {
            while let Some(key) = arg.next() {
                let key_info = self.inner_parser.lookup_short(key);
                match key_info {
                    LookupKey::Unknown => {
                        return Some(Err(ParseError::unknown_short_key(key).into()))
                    }
                    LookupKey::Single => {
                        if let Err(e) = self.inner_parser.parse_short(key) {
                            return Some(Err(e));
                        }
                    }
                    LookupKey::RequireValue(ty) => {
                        let value = match arg.as_str() {
                            "" => {
                                let Some(value) = self.raw_args.next() else {
                                    return Some(Err(ParseError::expected_value_short(key).into()));
                                };
                                value
                            }
                            v => v,
                        };

                        match ty {
                            KeyType::String => {
                                return Some(self.inner_parser.parse_short_str(key, value))
                            }
                            KeyType::Int => {
                                return Some(
                                    value
                                        .parse::<i64>()
                                        .map_err(|_| {
                                            ParseError::invalid_short_key_type(key, value, ty)
                                                .into()
                                        })
                                        .and_then(|value| {
                                            self.inner_parser.parse_short_int(key, value)
                                        }),
                                )
                            }
                            KeyType::UInt => {
                                return Some(
                                    value
                                        .parse::<u64>()
                                        .map_err(|_| {
                                            ParseError::invalid_short_key_type(key, value, ty)
                                                .into()
                                        })
                                        .and_then(|value| {
                                            self.inner_parser.parse_short_uint(key, value)
                                        }),
                                )
                            }
                        }
                    }
                }
            }
            Some(Ok(()))
        } else {
            Some(self.parse_raw(arg))
        }
    }

    pub fn parse_to_end(mut self) -> Result<T::Into, T::Error> {
        while let Some(res) = self.parse_once() {
            res?
        }
        self.inner_parser.parse_end()
    }
}
