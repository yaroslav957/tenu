#[cfg(test)]
mod tests {
    use tenu::lex::{ArgType, LookupTable, Parser, Token};

    #[test]
    fn grouped_short_with_value() {
        let args = ["./bin", "-hv", "-oout.jpg", "file1", "file2"]
            .iter()
            .copied();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
                ("output", ArgType::Required, 'o'),
            ]),
        );

        let tokens: Vec<_> = parser.collect();
        assert_eq!(
            tokens,
            Vec::from([
                Token::Value("./bin"),
                Token::Option("help", None),
                Token::Option("verbose", None),
                Token::Option("output", Some("out.jpg")),
                Token::Value("file1"),
                Token::Value("file2"),
            ])
        );
    }

    #[test]
    fn example_input() {
        let args = [
            "./aber",
            "-hv",
            "-ohedgeberry-s-mother.jpg",
            "-v",
            "log.txt",
            "l420.txt",
        ]
        .into_iter();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(
            Vec::from([
                Token::Value("./aber"),
                Token::Option("help", None),
                Token::Option("verbose", None),
                Token::Option("output", Some("hedgeberry-s-mother.jpg")),
                Token::Option("verbose", None),
                Token::Value("log.txt"),
                Token::Value("l420.txt"),
            ]),
            parser.collect::<Vec<_>>()
        )
    }

    #[test]
    fn mixed() {
        let args = ["test", "-ohedgeberry-s-mother.jpg", "-v", "log.txt"].into_iter();
        let parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
                ("verbose", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(
            Vec::from([
                Token::Value("test"),
                Token::Option("output", Some("hedgeberry-s-mother.jpg")),
                Token::Option("verbose", None),
                Token::Value("log.txt"),
            ]),
            parser.collect::<Vec<_>>()
        )
    }

    #[test]
    fn short_option_value_mixed() {
        let args = ["-v", "log.txt"].iter().copied();
        let mut parser = Parser::new(args, LookupTable(&[("verbose", ArgType::None, 'v')]));

        assert_eq!(parser.next(), Some(Token::Option("verbose", None)));
        assert_eq!(parser.next(), Some(Token::Value("log.txt")));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn short_option_dummy() {
        let args = ["-o-h"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(parser.next(), Some(Token::Option("output", Some("-h"))));
    }

    #[test]
    fn short_with_required_value() {
        let args = ["-o", "hedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(
            parser.next(),
            Some(Token::Option("output", Some("hedgeberry-s-mother.jpg")))
        );
    }

    #[test]
    fn short_with_required_value_squeezed() {
        let args = ["-ohedgeberry-s-mother.jpg"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("output", ArgType::Required, 'o'),
                ("help", ArgType::None, 'h'),
            ]),
        );
        assert_eq!(
            parser.next(),
            Some(Token::Option("output", Some("hedgeberry-s-mother.jpg")))
        );
    }

    #[test]
    fn short_squeezed() {
        let args = ["-hlv"].into_iter();
        let mut parser = Parser::new(
            args,
            LookupTable(&[
                ("help", ArgType::None, 'h'),
                ("list", ArgType::None, 'l'),
                ("version", ArgType::None, 'v'),
            ]),
        );
        assert_eq!(parser.next(), Some(Token::Option("help", None)));
        assert_eq!(parser.next(), Some(Token::Option("list", None)));
        assert_eq!(parser.next(), Some(Token::Option("version", None)));
    }

    #[test]
    fn short_without_value() {
        let args = ["-h"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::None, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_optional_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Option, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_with_optional_value_missing() {
        let args = ["--hello"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Option, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
    }

    #[test]
    fn long_with_required_value() {
        let args = ["--hello=world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::Required, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", Some("world"))));
    }

    #[test]
    fn long_flag_without_value() {
        let args = ["--hello", "world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[("hello", ArgType::None, 'h')]));
        assert_eq!(parser.next(), Some(Token::Option("hello", None)));
        assert_eq!(parser.next(), Some(Token::Value("world")));
    }

    #[test]
    fn dumb_test() {
        let args = ["hello", "world"].into_iter();
        let mut parser = Parser::new(args, LookupTable(&[]));
        assert_eq!(parser.next(), Some(Token::Value("hello")));
        assert_eq!(parser.next(), Some(Token::Value("world")));
    }
}
