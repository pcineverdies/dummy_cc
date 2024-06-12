use crate::lexer::lexer_impl::Token;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNode {
    AstNull,
    AstNumerical(Token),
    AstBoolean(Token),
    AstCharacter(Token),
    AstIdentifier(Token),
    AstStatements(Vec<AstNode>),
    AstAssignment(Box<AstNode>, Box<AstNode>),
    AstIf(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    AstFor(Box<AstNode>, Box<AstNode>, Box<AstNode>, Box<AstNode>),
    AstDecl(Token, Box<AstNode>, Box<AstNode>),
    AstWhile(Box<AstNode>, Box<AstNode>),
    AstFlow(Token, Box<AstNode>),
    AstExpression(Box<AstNode>),
    AstExpressionUnary(Token, Box<AstNode>),
    AstExpressionBinary(Token, Box<AstNode>, Box<AstNode>),
}

impl AstNode {
    pub fn new_null() -> AstNode {
        return AstNode::AstNull;
    }

    pub fn new_ast_numerical(tk: &Token) -> AstNode {
        return AstNode::AstNumerical(tk.clone());
    }

    pub fn new_ast_boolean(tk: &Token) -> AstNode {
        return AstNode::AstBoolean(tk.clone());
    }

    pub fn new_ast_character(tk: &Token) -> AstNode {
        return AstNode::AstCharacter(tk.clone());
    }

    pub fn new_ast_identifer(tk: &Token) -> AstNode {
        return AstNode::AstIdentifier(tk.clone());
    }

    pub fn new_ast_statements(van: &Vec<AstNode>) -> AstNode {
        return AstNode::AstStatements(van.to_vec());
    }

    pub fn new_ast_expr(an: &AstNode) -> AstNode {
        return AstNode::AstExpression(Box::new(an.clone()));
    }

    pub fn new_ast_assignment(an1: &AstNode, an2: &AstNode) -> AstNode {
        return AstNode::AstAssignment(Box::new(an1.clone()), Box::new(an2.clone()));
    }

    pub fn new_ast_if(an1: &AstNode, an2: &AstNode, an3: &AstNode) -> AstNode {
        return AstNode::AstIf(
            Box::new(an1.clone()),
            Box::new(an2.clone()),
            Box::new(an3.clone()),
        );
    }

    pub fn new_ast_for(an1: &AstNode, an2: &AstNode, an3: &AstNode, an4: &AstNode) -> AstNode {
        return AstNode::AstFor(
            Box::new(an1.clone()),
            Box::new(an2.clone()),
            Box::new(an3.clone()),
            Box::new(an4.clone()),
        );
    }

    pub fn new_ast_decl(an1: &Token, an2: &AstNode, an3: &AstNode) -> AstNode {
        return AstNode::AstDecl(an1.clone(), Box::new(an2.clone()), Box::new(an3.clone()));
    }

    pub fn new_ast_while(an1: &AstNode, an2: &AstNode) -> AstNode {
        return AstNode::AstWhile(Box::new(an1.clone()), Box::new(an2.clone()));
    }

    pub fn new_ast_flow(an1: &Token, an2: &AstNode) -> AstNode {
        return AstNode::AstFlow(an1.clone(), Box::new(an2.clone()));
    }

    pub fn new_ast_unary(an1: &Token, an2: &AstNode) -> AstNode {
        return AstNode::AstExpressionUnary(an1.clone(), Box::new(an2.clone()));
    }

    pub fn new_ast_binary(an1: &Token, an2: &AstNode, an3: &AstNode) -> AstNode {
        return AstNode::AstExpressionBinary(
            an1.clone(),
            Box::new(an2.clone()),
            Box::new(an3.clone()),
        );
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("");
        match self {
            AstNode::AstNull => {}
            AstNode::AstNumerical(value) => result += &value.tk.to_string(),
            AstNode::AstBoolean(value) => result += &value.tk.to_string(),
            AstNode::AstCharacter(value) => result += &value.tk.to_string(),
            AstNode::AstIdentifier(value) => result += &value.tk.to_string(),
            AstNode::AstStatements(value) => {
                for s in value {
                    result += &s.to_string();
                }
            }
            AstNode::AstAssignment(id, expr) => {
                result += &format!(
                    "{} = {};\n",
                    &id.to_string().as_str(),
                    &expr.to_string().as_str()
                );
            }
            AstNode::AstIf(expr, statements_if, statements_else) => {
                result += &format!(
                    "if({}){{\n{}",
                    &expr.to_string().as_str(),
                    &statements_if.to_string().as_str()
                );
                let else_print = statements_else.to_string();
                if else_print.len() > 0 {
                    result += &format!("}} else {{\n{}}}\n", else_print);
                }
            }
            AstNode::AstFor(decl, expr, ass, statements) => {
                result += &format!(
                    "for({};{};{}){{\n{}}}\n",
                    &decl.to_string().as_str(),
                    &expr.to_string().as_str(),
                    &ass.to_string().as_str(),
                    &statements.to_string().as_str()
                );
            }

            AstNode::AstDecl(decl, expr, ass) => {
                result += &format!(
                    "{} {} = {};\n",
                    &decl.tk.to_string().as_str(),
                    &expr.to_string().as_str(),
                    &ass.to_string().as_str()
                );
            }
            AstNode::AstWhile(expr, statements) => {
                result += &format!(
                    "while({}){{\n{}\n}}\n",
                    &expr.to_string().as_str(),
                    &statements.to_string().as_str()
                );
            }
            AstNode::AstFlow(kw, expr) => {
                result += &format!(
                    "{} {};\n",
                    kw.tk.to_string().as_str(),
                    expr.to_string().as_str()
                );
            }
            AstNode::AstExpression(expr) => {
                result += &format!("({})", expr.to_string().as_str());
            }
            AstNode::AstExpressionUnary(tk, expr) => {
                result += &format!(
                    "({}{})",
                    tk.tk.to_string().as_str(),
                    expr.to_string().as_str()
                );
            }
            AstNode::AstExpressionBinary(tk, expr1, expr2) => {
                result += &format!(
                    "({} {} {})",
                    expr1.to_string().as_str(),
                    tk.tk.to_string().as_str(),
                    expr2.to_string().as_str()
                );
            }
        }

        return result;
    }
}
