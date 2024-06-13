mod test {

    #[test]
    fn ast_test() {
        use crate::ast::ast_impl::AstNode;
        use crate::lexer::lexer_impl::{Keyword, Operator, Tk, Token};

        let ast_expr1 = AstNode::new_ast_identifer(&Token {
            tk: Tk::Identifier(String::from("a")),
            line_number: 0,
            character_number: 0,
        });

        let ast_expr4 = AstNode::new_ast_identifer(&Token {
            tk: Tk::Identifier(String::from("b")),
            line_number: 0,
            character_number: 0,
        });

        let ast_expr2 = AstNode::new_ast_numerical(&Token {
            tk: Tk::IntegerLiteral(3),
            line_number: 0,
            character_number: 0,
        });

        let ast_expr5 = AstNode::new_ast_numerical(&Token {
            tk: Tk::IntegerLiteral(6),
            line_number: 0,
            character_number: 0,
        });

        let ast_expr3 = AstNode::new_ast_binary(
            &Token {
                tk: Tk::Operator(Operator::Plus),
                line_number: 0,
                character_number: 0,
            },
            &ast_expr1,
            &ast_expr4,
        );

        let ast_expr7 = AstNode::new_ast_unary(
            &Token {
                tk: Tk::Operator(Operator::Minus),
                line_number: 0,
                character_number: 0,
            },
            &ast_expr4,
        );

        let ast_expr8 = AstNode::new_ast_binary(
            &Token {
                tk: Tk::Operator(Operator::Plus),
                line_number: 0,
                character_number: 0,
            },
            &ast_expr7,
            &ast_expr5,
        );

        let ast_decl = AstNode::new_ast_decl(
            &Token {
                tk: Tk::Keyword(Keyword::Int),
                line_number: 0,
                character_number: 0,
            },
            &ast_expr1,
            &ast_expr2,
        );

        let ast_block = AstNode::new_ast_statements(&vec![ast_decl.clone(), ast_decl.clone()]);

        let if_decl1 = AstNode::new_ast_if(&ast_expr3, &ast_block, &ast_block);
        let if_decl2 = AstNode::new_ast_if(&ast_expr8, &if_decl1, &ast_block);

        println!("{}", if_decl2.to_string(0));

        assert_eq!(
            if_decl2.to_string(0),
            "if(((-b) + 6)){
  if((a + b)){
    int a = 3;
    int a = 3;
  } else {
    int a = 3;
    int a = 3;
  }} else {
  int a = 3;
  int a = 3;
}"
        );
    }
}
