use tenu::lex::{KeyType, LookupKey, ParseError, ParseInto, ParseIntoExt as _};

enum ArgType {
    None,
    Required,
    Option,
}

struct LookupTable(&'static [(&'static str, ArgType, char)]);

struct LookupTableHS {
    pub hs: Vec<(String, String)>,
    pub options: LookupTable,
}

impl<'a> ParseInto<'a> for LookupTableHS {
    type Into = Vec<(String, String)>;
    type Error = ParseError<'a>;

    fn lookup_raw(&self) -> Option<KeyType> {
        // Allow all raw arguments
        Some(KeyType::String)
    }
    fn lookup_short(&self, short: char) -> LookupKey {
        match self.options.0.iter().find(|p| p.2 == short) {
            Some((_, ArgType::None, _)) => LookupKey::Single,
            Some(_) => LookupKey::RequireValue(KeyType::String),
            None => LookupKey::Unknown,
        }
    }
    fn lookup_long(&self, long: &str) -> LookupKey {
        match self.options.0.iter().find(|p| p.0 == long) {
            Some((_, ArgType::None, _)) => LookupKey::Single,
            Some(_) => LookupKey::RequireValue(KeyType::String),
            None => LookupKey::Unknown,
        }
    }

    fn parse_short(&mut self, short: char) -> Result<(), Self::Error> {
        self.hs.push((format!("[short]{short}"), "[none]".into()));
        Ok(())
    }
    fn parse_short_str(&mut self, short: char, value: &'a str) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[short]{short}"), format!("[str]{value}")));
        Ok(())
    }
    fn parse_short_int(&mut self, short: char, value: i64) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[short]{short}"), format!("[i64]{value}")));
        Ok(())
    }
    fn parse_short_uint(&mut self, short: char, value: u64) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[short]{short}"), format!("[u64]{value}")));
        Ok(())
    }

    fn parse_long(&mut self, long: &str) -> Result<(), Self::Error> {
        self.hs.push((format!("[long]{long}"), "[none]".into()));
        Ok(())
    }
    fn parse_long_str(&mut self, long: &'a str, value: &'a str) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[long]{long}"), format!("[str]{value}")));
        Ok(())
    }
    fn parse_long_int(&mut self, long: &'a str, value: i64) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[long]{long}"), format!("[i64]{value}")));
        Ok(())
    }
    fn parse_long_uint(&mut self, long: &'a str, value: u64) -> Result<(), Self::Error> {
        self.hs
            .push((format!("[long]{long}"), format!("[u64]{value}")));
        Ok(())
    }

    fn parse_raw_str(&mut self, raw: &'a str) -> Result<(), Self::Error> {
        self.hs.push((format!("[raw]"), raw.into()));
        Ok(())
    }
    fn parse_raw_int(&mut self, raw: i64) -> Result<(), Self::Error> {
        self.hs.push((format!("[raw][i64]"), raw.to_string()));
        Ok(())
    }
    fn parse_raw_uint(&mut self, raw: u64) -> Result<(), Self::Error> {
        self.hs.push((format!("[raw][u64]"), raw.to_string()));
        Ok(())
    }

    fn parse_end(self) -> Result<Self::Into, Self::Error> {
        // check idk
        Ok(self.hs)
    }
}

const OPTIONS: LookupTable = LookupTable(&[
    ("help", ArgType::None, 'h'),
    ("output", ArgType::Required, 'o'),
    ("verbose", ArgType::None, 'v'),
]);

fn main() {
    let parser = LookupTableHS {
        hs: Vec::new(),
        options: OPTIONS,
    };
    println!(
        "{:#?}",
        parser.parse(
            [
                "-hv",
                "-ohedgeberry-s-mother.jpg",
                "-v",
                "log.txt",
                "l420.txt",
                "--",
                "--zov",
            ]
            .into_iter()
        )
    );
}
