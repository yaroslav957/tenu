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

fn main() {
    let _args = tenu::args();

    // API:
    //
    let mut parser = Parser::new(_args, OPTIONS);
    //    let mut parser = Parser::new(
    //        &[
    //            "-hlv",
    //            "-ohedgeberry-s-mother.jpg",
    //            "-v",
    //            "log.txt",
    //            "l420.txt",
    //            "--",
    //            "--zov",
    //        ],
    //        OPTIONS,
    //    );

    dbg!(parser.parse().unwrap());
}
