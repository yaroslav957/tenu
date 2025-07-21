use tenu::{
    args,
    lex::{ArgType, LookupTable, Parser},
};

const OPTIONS: LookupTable = LookupTable(&[
    ("help", ArgType::None, 'h'),
    ("output", ArgType::Required, 'o'),
    ("list", ArgType::None, 'l'),
    ("verbose", ArgType::Option, 'v'),
]);

fn main() {}
