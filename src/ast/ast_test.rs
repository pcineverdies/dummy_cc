mod test {

    #[test]
    fn ast_test() {
        use crate::ast::ast_impl::{AstNode, AstNodeWrapper, TypeNative, TypeWrapper};
        use crate::lexer::lexer_impl::{Operator, Tk, Token};

        let ast_expr1 = AstNodeWrapper {
            node: AstNode::new_primary(&Token {
                tk: Tk::Identifier(String::from("a")),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            }),
            ..Default::default()
        };

        let ast_expr4 = AstNodeWrapper {
            node: AstNode::new_primary(&Token {
                tk: Tk::Identifier(String::from("b")),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            }),
            ..Default::default()
        };

        let ast_expr2 = AstNodeWrapper {
            node: AstNode::new_primary(&Token {
                tk: Tk::IntegerLiteral(3),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            }),
            ..Default::default()
        };

        let ast_expr5 = AstNodeWrapper {
            node: AstNode::new_primary(&Token {
                tk: Tk::IntegerLiteral(6),
                line_number: 0,
                last_character: 0,
                first_character: 0,
            }),
            ..Default::default()
        };

        let ast_type = AstNodeWrapper {
            node: AstNode::new_type(&TypeWrapper {
                constant: true,
                type_native: TypeNative::I8,
                pointer: 3,
            }),
            ..Default::default()
        };

        let ast_expr51 = AstNodeWrapper {
            node: AstNode::new_cast(&ast_type, &ast_expr5),
            ..Default::default()
        };

        let ast_expr3 = AstNodeWrapper {
            node: AstNode::new_binary(
                &Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 0,
                    last_character: 0,
                    first_character: 0,
                },
                &ast_expr1,
                &ast_expr4,
            ),
            ..Default::default()
        };

        let ast_expr7 = AstNodeWrapper {
            node: AstNode::new_prefix(
                &Token {
                    tk: Tk::Operator(Operator::Minus),
                    line_number: 0,
                    last_character: 0,
                    first_character: 0,
                },
                &ast_expr4,
            ),
            ..Default::default()
        };

        let ast_expr8 = AstNodeWrapper {
            node: AstNode::new_binary(
                &Token {
                    tk: Tk::Operator(Operator::Plus),
                    line_number: 0,
                    last_character: 0,
                    first_character: 0,
                },
                &ast_expr7,
                &ast_expr51,
            ),
            ..Default::default()
        };

        let ast_decl = AstNodeWrapper {
            node: AstNode::new_var_decl(
                &ast_type,
                &Token {
                    tk: Tk::Identifier(String::from("var")),
                    line_number: 0,
                    last_character: 0,
                    first_character: 0,
                },
                &ast_expr2,
            ),
            ..Default::default()
        };

        let ast_block1 = AstNodeWrapper {
            node: AstNode::new_compound(&vec![ast_decl.clone(), ast_decl.clone()]),
            ..Default::default()
        };
        let if_decl1 = AstNodeWrapper {
            node: AstNode::new_if(&ast_expr3, &ast_block1, &AstNodeWrapper { ..Default::default() }),
            ..Default::default()
        };
        let ast_block2 = AstNodeWrapper {
            node: AstNode::new_compound(&vec![if_decl1.clone()]),
            ..Default::default()
        };
        let if_decl2 = AstNodeWrapper {
            node: AstNode::new_if(&ast_expr8, &ast_block2, &ast_block1),
            ..Default::default()
        };

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
