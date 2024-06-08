mod test {

    #[test]
    fn lx_test_comment() {
        use crate::lexer::lexer_impl::{Bracket, Keyword, Lexer, Tk, Token};

        let input_code = "int
            // This comment will be ignored
            main
            ( // This comment is ignored
            // This comment is ignored as well
            ){}
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::Keyword(Keyword::Int),
                    line_number: 1
                },
                Token {
                    tk: Tk::Keyword(Keyword::Main),
                    line_number: 3
                },
                Token {
                    tk: Tk::Bracket(Bracket::LBracket),
                    line_number: 4
                },
                Token {
                    tk: Tk::Bracket(Bracket::RBracket),
                    line_number: 6
                },
                Token {
                    tk: Tk::Bracket(Bracket::LCurly),
                    line_number: 6
                },
                Token {
                    tk: Tk::Bracket(Bracket::RCurly),
                    line_number: 6
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 7
                },
            ]
        );
    }

    #[test]
    fn lx_test_operators() {
        use crate::lexer::lexer_impl::{Lexer, Operator, Tk, Token};

        let input_code = "==+-/>=k&^
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::Operator(Operator::EqualCompare),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::Minus),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::Slash),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::GECompare),
                    line_number: 1
                },
                Token {
                    tk: Tk::Identifier("k".to_string()),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::And),
                    line_number: 1
                },
                Token {
                    tk: Tk::Operator(Operator::Xor),
                    line_number: 1
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 2
                },
            ]
        );
    }

    #[test]
    fn lx_test_open_string() {
        use crate::lexer::lexer_impl::{Lexer, Operator, Tk, Token};

        let input_code = "
            \"daje
    ++
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::ERROR,
                    line_number: 2
                },
                Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 2
                },
                Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 2
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 3
                },
            ]
        );
    }

    #[test]
    fn lx_test_numbers() {
        use crate::lexer::lexer_impl::{Lexer, Tk, Token};

        let input_code = "
            0x10
            10
            010
            0b10
            080 0xer
            // comment
            d0x20
";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::IntegerLiteral(16),
                    line_number: 2
                },
                Token {
                    tk: Tk::IntegerLiteral(10),
                    line_number: 3
                },
                Token {
                    tk: Tk::IntegerLiteral(8),
                    line_number: 4
                },
                Token {
                    tk: Tk::IntegerLiteral(2),
                    line_number: 5
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 6
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 6
                },
                Token {
                    tk: Tk::Identifier("d0x20".to_string()),
                    line_number: 8
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 9
                },
            ]
        );
    }

    #[test]
    fn lx_test_characters() {
        use crate::lexer::lexer_impl::{Lexer, Tk, Token};

        let input_code = "
    'd'
    'e'
    '  '

";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::Char('d'),
                    line_number: 2
                },
                Token {
                    tk: Tk::Char('e'),
                    line_number: 3
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 4
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 4
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 6
                },
            ]
        );
    }

    #[test]
    fn lx_test_invalid() {
        use crate::lexer::lexer_impl::{Lexer, Tk, Token};

        let input_code = "
    'd'
    'e'
    '  '

";

        assert_eq!(
            Lexer::new(input_code.to_string(), false)
                .unwrap()
                .tokenize(),
            &[
                Token {
                    tk: Tk::Char('d'),
                    line_number: 2
                },
                Token {
                    tk: Tk::Char('e'),
                    line_number: 3
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 4
                },
                Token {
                    tk: Tk::ERROR,
                    line_number: 4
                },
                Token {
                    tk: Tk::EOF,
                    line_number: 6
                },
            ]
        );
    }
}
