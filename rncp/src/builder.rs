use crate::{Args, parser::ValueParser};

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

    pub fn get(self, args: &Args<'a>) -> T {
        let value = args
            .0
            .iter()
            .skip_while(|n| n.0 != self.name)
            .skip(1)
            .next()
            .unwrap();

        self.parser.unwrap().parse(value.0)
    }
}
