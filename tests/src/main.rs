use tenu::{
    Env,
    lex::{ArgType, LookupTable, Parser},
};

const OPTIONS: LookupTable = LookupTable(&[
    ("help", ArgType::None, 'h'),
    ("output", ArgType::Required, 'o'),
    ("verbose", ArgType::Option, 'v'),
]);

fn main() {
    let _args = Env::args();

    // API:
    //
    // Parser::new(OPTIONS).parse(src);
    let mut parser = Parser::new(
        &[
            "-h",
            "-ohedgeberry-s-mother.jpg",
            "-v",
            "log.txt",
            "l420.txt",
            "--",
            "--zov",
        ],
        OPTIONS,
    );

    dbg!(parser.parse().unwrap());
}
