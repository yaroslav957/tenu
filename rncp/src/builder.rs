// Фул переписать лол
// UPD: rewrote ur mother
// UPD2: OCD moment bro, it lgtm

use alloc::vec::Vec;

use crate::{
    env::Args,
    error::{Error, Result},
    parser::ValueParser,
};

pub struct ArgBuilder<'a, T> {
    /// Argument name.
    name: &'static str,

    /// Value parser
    parser: Option<&'a dyn ValueParser<T>>,

    /// A command aliases
    aliases: Vec<&'static str>,

    /// Whether to create a short version of the command,
    /// for exapmle, for a `--name` command `-n` would be created
    short: bool,
}

// What u think about using `where`? Just a question
// UPD: I don't see any needs for `where` lol
// Dolbabe? why not make `where` everywhere = читаемый, идиоматичный код
impl<'a, T> ArgBuilder<'a, T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            parser: None,
            aliases: Vec::with_capacity(0),
            short: false,
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

    pub fn alias(mut self, name: &'static str) -> Self {
        self.aliases.push(name);
        self
    }

    // TODO: Rewrite
    // And it's not supposed to be here because it's not a builder responsibility
    // to be honest
    // Я насрал. переделай
    pub fn get(&self, args: &Args) -> Result<T> {
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
