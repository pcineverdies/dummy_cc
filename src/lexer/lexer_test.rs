mod test {

    #[test]
    fn lx_test_comment() {
        use crate::lexer::lexer_impl::{Bracket, Keyword, Lexer, Tk, Token};

        let input_code = "u16
            // This comment will be ignored
            main
            ( // This comment is ignored
            // This comment is ignored as well
            ){}
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false).unwrap().tokenize().unwrap(),
            &[
                Token {
                    tk: Tk::Keyword(Keyword::U16),
                    line_number: 1,
                    last_character: 3,
                    first_character: 1
                },
                Token {
                    tk: Tk::Identifier(String::from("main")),
                    line_number: 3,
                    last_character: 16,
                    first_character: 13,
                },
                Token {
                    tk: Tk::Bracket(Bracket::LBracket),
                    line_number: 4,
                    last_character: 13,
                    first_character: 13,
                },
                Token {
                    tk: Tk::Bracket(Bracket::RBracket),
                    line_number: 6,
                    first_character: 13,
                    last_character: 13,
                },
                Token {
                    tk: Tk::Bracket(Bracket::LCurly),
                    line_number: 6,
                    last_character: 14,
                    first_character: 14,
                },
                Token {
                    tk: Tk::Bracket(Bracket::RCurly),
                    line_number: 6,
                    last_character: 15,
                    first_character: 15,
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 7,
                    last_character: 2,
                    first_character: 1,
                },
            ]
        );
    }

    #[test]
    fn lx_test_operators() {
        use crate::lexer::lexer_impl::{Lexer, Operator, Tk, Token};

        let input_code = "==+-/>=k&^!=
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false).unwrap().tokenize().unwrap(),
            &[
                Token {
                    tk: Tk::Operator(Operator::EqualCompare),
                    line_number: 1,
                    first_character: 1,
                    last_character: 2,
                },
                Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 1,
                    first_character: 3,
                    last_character: 3,
                },
                Token {
                    tk: Tk::Operator(Operator::Minus),
                    line_number: 1,
                    first_character: 4,
                    last_character: 4,
                },
                Token {
                    tk: Tk::Operator(Operator::Slash),
                    line_number: 1,
                    first_character: 5,
                    last_character: 5,
                },
                Token {
                    tk: Tk::Operator(Operator::GECompare),
                    line_number: 1,
                    first_character: 6,
                    last_character: 7,
                },
                Token {
                    tk: Tk::Identifier("k".to_string()),
                    line_number: 1,
                    last_character: 8,
                    first_character: 8,
                },
                Token {
                    tk: Tk::Operator(Operator::AndOp),
                    line_number: 1,
                    last_character: 9,
                    first_character: 9,
                },
                Token {
                    tk: Tk::Operator(Operator::XorOp),
                    line_number: 1,
                    last_character: 10,
                    first_character: 10,
                },
                Token {
                    tk: Tk::Operator(Operator::DiffCompare),
                    line_number: 1,
                    last_character: 12,
                    first_character: 11,
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 2,
                    last_character: 2,
                    first_character: 1,
                },
            ]
        );
    }

    #[test]
    fn lx_test_open_string() {
        use crate::lexer::lexer_impl::Lexer;

        let input_code = "
            \"daje
    ++
";

        assert!(Lexer::new(input_code.to_string(), false).unwrap().tokenize().is_none());
    }

    #[test]
    fn lx_test_numbers() {
        use crate::lexer::lexer_impl::Lexer;

        let input_code = "
            0x10
            10
            010
            0b10
            080 0xer
            // comment
            d0x20
            break
";

        assert!(Lexer::new(input_code.to_string(), false).unwrap().tokenize().is_none());
    }

    #[test]
    fn lx_test_characters() {
        use crate::lexer::lexer_impl::{Lexer, Tk, Token};

        let input_code = "
    'd'
    'e'


";

        assert_eq!(
            Lexer::new(input_code.to_string(), false).unwrap().tokenize().unwrap(),
            &[
                Token {
                    tk: Tk::Char('d'),
                    line_number: 2,
                    last_character: 7,
                    first_character: 5,
                },
                Token {
                    tk: Tk::Char('e'),
                    line_number: 3,
                    last_character: 7,
                    first_character: 5,
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 6,
                    last_character: 2,
                    first_character: 1,
                },
            ]
        );
    }

    #[test]
    fn lx_test_invalid() {
        use crate::lexer::lexer_impl::Lexer;

        let input_code = "
    'd'
    'e'
    '  '

";

        assert!(Lexer::new(input_code.to_string(), false).unwrap().tokenize().is_none());
    }
}
