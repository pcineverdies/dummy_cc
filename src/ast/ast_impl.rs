use crate::lexer::lexer_impl::Token;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    fn get_indent(&self, indent: u32) -> String {
        let mut result = String::from("");
        for _ in 0..indent {
            result += "  ";
        }
        result
    }

    pub fn to_string(&self, indent: u32) -> String {
        let mut result = String::from("");
        match self {
            AstNode::AstStatements(_) => {}
            _ => result += &self.get_indent(indent),
        }
        match self {
            AstNode::AstNull => {}
            AstNode::AstNumerical(value) => result += &value.tk.to_string(),
            AstNode::AstBoolean(value) => result += &value.tk.to_string(),
            AstNode::AstCharacter(value) => result += &value.tk.to_string(),
            AstNode::AstIdentifier(value) => result += &value.tk.to_string(),
            AstNode::AstStatements(value) => {
                for i in 0..value.len() {
                    let s = &value[i];
                    match s {
                        AstNode::AstAssignment(_, _) => {
                            result += &s.to_string(indent);
                            result += ";";
                        }
                        _ => {
                            result += &s.to_string(indent);
                        }
                    }
                    result += "\n";
                }
            }
            AstNode::AstAssignment(id, expr) => {
                result += &format!(
                    "{} = {}",
                    &id.to_string(0).as_str(),
                    &expr.to_string(0).as_str()
                );
            }
            AstNode::AstIf(expr, statements_if, statements_else) => {
                result += &format!(
                    "if({}){{\n{}",
                    &expr.to_string(0).as_str(),
                    &statements_if.to_string(indent + 1).as_str()
                );
                let else_print = statements_else.to_string(indent + 1);
                if else_print.len() as u32 > (indent as u32 + 1) * 2 {
                    result += &self.get_indent(indent);
                    result += &format!("}} else {{\n{}", else_print);
                }
                result += &self.get_indent(indent);
                result += &format!("}}");
            }
            AstNode::AstFor(decl, expr, ass, statements) => {
                result += &format!(
                    "for({} {}; {}){{\n{}",
                    &decl.to_string(0).as_str(),
                    &expr.to_string(0).as_str(),
                    &ass.to_string(0).as_str(),
                    &statements.to_string(indent + 1).as_str()
                );
                result += &self.get_indent(indent);
                result += &format!("}}");
            }

            AstNode::AstDecl(decl, expr, ass) => {
                result += &format!(
                    "{} {} = {};",
                    &decl.tk.to_string().as_str(),
                    &expr.to_string(0).as_str(),
                    &ass.to_string(0).as_str()
                );
            }
            AstNode::AstWhile(expr, statements) => {
                result += &format!(
                    "while({}){{\n{}",
                    &expr.to_string(0).as_str(),
                    &statements.to_string(indent + 1).as_str()
                );
                result += &self.get_indent(indent);
                result += &format!("}}");
            }
            AstNode::AstFlow(kw, expr) => match **expr {
                AstNode::AstNull => {
                    result += &format!("{};", kw.tk.to_string().as_str(),);
                }
                _ => {
                    result += &format!(
                        "{} {};",
                        kw.tk.to_string().as_str(),
                        expr.to_string(0).as_str()
                    );
                }
            },
            AstNode::AstExpressionUnary(tk, expr) => {
                result += &format!(
                    "({}{})",
                    tk.tk.to_string().as_str(),
                    expr.to_string(0).as_str()
                );
            }
            AstNode::AstExpressionBinary(tk, expr1, expr2) => {
                result += &format!(
                    "({} {} {})",
                    expr1.to_string(0).as_str(),
                    tk.tk.to_string().as_str(),
                    expr2.to_string(0).as_str()
                );
            }
        }

        return result;
    }
}
