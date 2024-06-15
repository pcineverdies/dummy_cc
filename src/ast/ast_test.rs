mod test {

    #[test]
    fn ast_test() {
        use crate::ast::ast_impl::AstNode;
        use crate::lexer::lexer_impl::{Keyword, Operator, Tk, Token};

        let ast_expr1 = AstNode::new_primary(&Token {
            tk: Tk::Identifier(String::from("a")),
            line_number: 0,
            last_character: 0,
            first_character: 0,
        });

        let ast_expr4 = AstNode::new_primary(&Token {
            tk: Tk::Identifier(String::from("b")),
            line_number: 0,
            last_character: 0,
            first_character: 0,
        });

        let ast_expr2 = AstNode::new_primary(&Token {
            tk: Tk::IntegerLiteral(3),
            line_number: 0,
            last_character: 0,
            first_character: 0,
        });

        let ast_expr5 = AstNode::new_primary(&Token {
            tk: Tk::IntegerLiteral(6),
            line_number: 0,
            last_character: 0,
            first_character: 0,
        });

        let ast_type = AstNode::new_type(
            true,
            &Token {
                tk: Tk::Keyword(Keyword::I8),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            },
            3,
        );

        let ast_expr51 = AstNode::new_cast(&ast_type, &ast_expr5);

        let ast_expr3 = AstNode::new_binary(
            &Token {
                tk: Tk::Operator(Operator::Plus),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            },
            &ast_expr1,
            &ast_expr4,
        );

        let ast_expr7 = AstNode::new_prefix(
            &Token {
                tk: Tk::Operator(Operator::Minus),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            },
            &ast_expr4,
        );

        let ast_expr8 = AstNode::new_binary(
            &Token {
                tk: Tk::Operator(Operator::Plus),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            },
            &ast_expr7,
            &ast_expr51,
        );

        let ast_decl = AstNode::new_var_decl(
            &ast_type,
            &Token {
                tk: Tk::Identifier(String::from("var")),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            },
            &ast_expr2,
        );

        let ast_block1 = AstNode::new_compound(&vec![ast_decl.clone(), ast_decl.clone()]);
        let if_decl1 = AstNode::new_if(&ast_expr3, &ast_block1, &AstNode::new_null());
        let ast_block2 = AstNode::new_compound(&vec![if_decl1.clone()]);
        let if_decl2 = AstNode::new_if(&ast_expr8, &ast_block2, &ast_block1);

        println!("{}", if_decl2.to_string(0));

        assert_eq!(
            if_decl2.to_string(0),
            "if(((-b) + ((const i8***)6))){
  if((a + b)){
    const i8*** var = 3;
    const i8*** var = 3;
  }
}
else{
  const i8*** var = 3;
  const i8*** var = 3;
}
"
        );
    }
}
