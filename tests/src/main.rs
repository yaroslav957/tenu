use tenu::lex::{LookupTable, Parser, CliOption, ArgType};

static OPTIONS: &[CliOption] = &[
    ("help", ArgType::None, 'h'),
    ("output", ArgType::Required, 'o'),
    ("verbose", ArgType::Option, 'v'),
];

// Please do not remove these tests (!!!), but you can always move them into tenu_parser, I wouldn't mind :p
fn main() {
    let mut parser = Parser::new(
        &["-h", "-ohedgeberry-s-mother.jpg", "-v", "log.txt", "l420.txt", "--", "--zov"],
        LookupTable(OPTIONS)
    );
    dbg!(parser.parse());
}

