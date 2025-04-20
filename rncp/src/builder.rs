// Фул переписать лол

use crate::{
    env::Args,
    error::{Error, Result},
    parser::ValueParser,
};

pub struct ArgBuilder<'a, T> {
    name: &'static str,
    // add `aliases` field
    parser: Option<&'a dyn ValueParser<T>>,
    short: bool,
}

// What u think about using `where`? Just a question
impl<'a, T> ArgBuilder<'a, T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            parser: None,
            short: false, // ???
        }
    }

    pub fn with_value(mut self, parser: &'a dyn ValueParser<T>) -> Self {
        self.parser = Some(parser);
        self
    }

    pub fn short(mut self) -> Self {
        self.short = true;
        self
    }
    
    // переписать этот бред
    pub fn get(&self, args: &Args<'a>) -> Result<T> {
        match args.0.iter().enumerate().find(|&(_, n)| *n == self.name) {
            Some((p, _)) if p + 1 < args.0.len() => {
                let value_str = &args.0[p + 1];
                let parser = self.parser.ok_or(Error::DuplicatedArg)?; // затычка пока не сделаю норм ерори
                parser.parse(value_str)
            }
            Some(_) => Err(Error::MissingArgValue),
            None => Err(Error::MissingArg),
        }
    }
}
