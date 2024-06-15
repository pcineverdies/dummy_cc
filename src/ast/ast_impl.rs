use crate::lexer::lexer_impl::Token;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNode {
    NullNode,
    PrimaryNode(Token),
    BinaryNode(Token, Box<AstNode>, Box<AstNode>),
    PrefixNode(Token, Box<AstNode>),
    CastNode(Box<AstNode>, Box<AstNode>),
    TypeNode(Token, u32),
    ProcedureNode(Box<AstNode>, Vec<AstNode>),
    SelectorNode(Box<AstNode>, Vec<AstNode>),
    CompoundNode(Vec<AstNode>),
    ExprStatementNode(Box<AstNode>),
    IfNode(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    WhileNode(Box<AstNode>, Box<AstNode>),
    ForNode(Box<AstNode>, Box<AstNode>, Box<AstNode>, Box<AstNode>),
    JumpNode(Token, Box<AstNode>),
    VarDeclNode(Box<AstNode>, Token, Box<AstNode>),
    ParameterNode(Box<AstNode>, Token),
    FuncDeclNode(Box<AstNode>, Token, Vec<AstNode>, Box<AstNode>),
    ArrayDeclNode(Box<AstNode>, Token, Vec<AstNode>),
}

use AstNode::*;

impl AstNode {

    /// AstNode::new_null
    ///
    /// Create a NullNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_null() -> AstNode {
        NullNode
    }

    /// AstNode::new_primary
    ///
    /// Create a PrimaryNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_primary(tk: &Token) -> AstNode {
        PrimaryNode(tk.clone())
    }

    /// AstNode::new_binary
    ///
    /// Create a BinaryNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_binary(an1: &Token, an2: &AstNode, an3: &AstNode) -> AstNode {
        BinaryNode(an1.clone(), Box::new(an2.clone()), Box::new(an3.clone()))
    }

    /// AstNode::new_prefix
    ///
    /// Create a PrefixNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_prefix(an1: &Token, an2: &AstNode) -> AstNode {
        PrefixNode(an1.clone(), Box::new(an2.clone()))
    }

    /// AstNode::new_cast
    ///
    /// Create a CastNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_cast(an1: &AstNode, an2: &AstNode) -> AstNode {
        CastNode(Box::new(an1.clone()), Box::new(an2.clone()))
    }

    /// AstNode::new_type
    ///
    /// Create a TypeNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_type(an1: &Token, an2: u32) -> AstNode {
        TypeNode(an1.clone(), an2)
    }

    /// AstNode::new_procedure
    ///
    /// Create a ProcedureNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_procedure(an1: &AstNode, an2: &Vec<AstNode>) -> AstNode {
        ProcedureNode(Box::new(an1.clone()), an2.clone())
    }

    /// AstNode::new_selector
    ///
    /// Create a SelectorNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_selector(an1: &AstNode, an2: &Vec<AstNode>) -> AstNode {
        SelectorNode(Box::new(an1.clone()), an2.clone())
    }

    /// AstNode::new_compound
    ///
    /// Create a CompoundNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_compound(an1: &Vec<AstNode>) -> AstNode {
        CompoundNode(an1.clone())
    }

    /// AstNode::new_expr_statement
    ///
    /// Create a ExprStatementNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_expr_statement(an1: &AstNode) -> AstNode {
        ExprStatementNode(Box::new(an1.clone()))
    }

    /// AstNode::new_if
    ///
    /// Create a IfNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_if(an1: &AstNode, an2: &AstNode, an3: &AstNode) -> AstNode {
        IfNode(
            Box::new(an1.clone()),
            Box::new(an2.clone()),
            Box::new(an3.clone()),
        )
    }

    /// AstNode::new_while
    ///
    /// Create a WhileNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_while(an1: &AstNode, an2: &AstNode) -> AstNode {
        WhileNode(Box::new(an1.clone()), Box::new(an2.clone()))
    }

    /// AstNode::new_for
    ///
    /// Create a ForNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_for(an1: &AstNode, an2: &AstNode, an3: &AstNode, an4: &AstNode) -> AstNode {
        ForNode(
            Box::new(an1.clone()),
            Box::new(an2.clone()),
            Box::new(an3.clone()),
            Box::new(an4.clone()),
        )
    }

    /// AstNode::new_jump
    ///
    /// Create a JumpNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_jump(an1: &Token, an2: &AstNode) -> AstNode {
        JumpNode(an1.clone(), Box::new(an2.clone()))
    }

    /// AstNode::new_var_decl
    ///
    /// Create a VarDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_var_decl(an1: &AstNode, an2: &Token, an3: &AstNode) -> AstNode {
        VarDeclNode(Box::new(an1.clone()), an2.clone(), Box::new(an3.clone()))
    }

    /// AstNode::new_func_decl
    ///
    /// Create a FuncDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_func_decl(an1: &AstNode, an2: &Token, an3: &Vec<AstNode>, an4: &AstNode) -> AstNode {
        FuncDeclNode(
            Box::new(an1.clone()),
            an2.clone(),
            an3.clone(),
            Box::new(an4.clone()),
        )
    }

    /// AstNode::new_array_decl
    ///
    /// Create a ArrayDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_array_decl(an1: &AstNode, an2: &Token, an3: &Vec<AstNode>) -> AstNode {
        ArrayDeclNode(Box::new(an1.clone()), an2.clone(), an3.clone())
    }

    fn get_indent(&self, indent: u32) -> String {
        let mut result = String::from("");
        for _ in 0..indent {
            result += "  ";
        }
        result
    }

    /// AstNode::to_string
    ///
    /// Transform the current AstNode to a string, exploiting the function in a recursive fashion.
    /// The parameter indent is used to indicate how much to indent, so that the final result is
    /// consistent with a readable formato of the code
    ///
    /// @in indent[u32] How much to indent
    /// @return [String] Result of the string conversion
    pub fn to_string(&self, indent: u32) -> String {
        let mut result = String::from("");
        match self {
            AstNode::CompoundNode(_) => {}
            _ => result += &self.get_indent(indent),
        }
        match self {
            NullNode => {}
            PrimaryNode(value) => result += &value.tk.to_string(),
            BinaryNode(tk, expr1, expr2) => {
                result += &format!(
                    "({} {} {})",
                    expr1.to_string(0).as_str(),
                    tk.tk.to_string().as_str(),
                    expr2.to_string(0).as_str()
                );
            }
            PrefixNode(tk, expr) => {
                result += &format!(
                    "({}{})",
                    tk.tk.to_string().as_str(),
                    expr.to_string(0).as_str()
                );
            }
            CastNode(cn, expr) => {
                result += &format!(
                    "(({}){})",
                    cn.to_string(0).as_str(),
                    expr.to_string(0).as_str()
                );
            }
            TypeNode(tt, counter) => {
                result += &format!("{}", tt.tk.to_string().as_str(),);
                for _ in 0..*counter {
                    result += "*";
                }
            }
            ProcedureNode(expr, args) => {
                result += &format!("({})(", expr.to_string(0).as_str(),);
                for i in 0..args.len() {
                    result += &format!("{}", args[i].to_string(0).as_str(),);
                    if i != args.len() - 1 {
                        result += ",";
                    }
                }
                result += ")";
            }
            SelectorNode(expr, args) => {
                result += &format!("({})", expr.to_string(0).as_str(),);
                for i in 0..args.len() {
                    result += &format!("[{}]", args[i].to_string(0).as_str(),);
                }
            }
            CompoundNode(value) => {
                result += "{\n";
                for s in value {
                    result += &s.to_string(indent + 1);
                }
                result += &self.get_indent(indent);
                result += "}\n";
            }
            ExprStatementNode(expr) => {
                if **expr != NullNode {
                    result += &format!("{}", expr.to_string(0).as_str(),);
                }
                result += ";\n";
            }
            IfNode(expr, statements_if, statements_else) => {
                result += &format!(
                    "if({}) {}",
                    &expr.to_string(0).as_str(),
                    &statements_if.to_string(indent).as_str()
                );
                let else_print = statements_else.to_string(indent);
                if else_print.len() as u32 > (indent as u32) * 2 {
                    result += &self.get_indent(indent);
                    result += &format!("else {}", else_print);
                }
            }
            WhileNode(expr, statements) => {
                result += &format!(
                    "while({}) {}",
                    &expr.to_string(0).as_str(),
                    &statements.to_string(indent).as_str()
                );
            }
            ForNode(decl, expr, ass, statements) => {
                result += &format!(
                    "for({}; {}; {}) {}",
                    &decl.to_string(0).as_str(),
                    &expr.to_string(0).as_str(),
                    &ass.to_string(0).as_str(),
                    &statements.to_string(indent).as_str()
                );
            }
            JumpNode(kw, expr) => match **expr {
                NullNode => {
                    result += &format!("{};\n", kw.tk.to_string().as_str(),);
                }
                _ => {
                    result += &format!(
                        "{} {};\n",
                        kw.tk.to_string().as_str(),
                        expr.to_string(0).as_str()
                    );
                }
            },
            VarDeclNode(tt, id, expr) => {
                result += &format!(
                    "{} {}",
                    tt.to_string(0).as_str(),
                    id.tk.to_string().as_str()
                );
                if **expr == NullNode {
                    result += &format!(";\n");
                } else {
                    result += &format!("{};\n", expr.to_string(0).as_str(),);
                }
            }
            FuncDeclNode(tt, id, args, body) => {
                result += &format!(
                    "{} {}(",
                    tt.to_string(0).as_str(),
                    id.tk.to_string().as_str()
                );
                for i in 0..args.len() {
                    result += &format!("{}", args[i].to_string(0).as_str(),);
                    if i != args.len() - 1 {
                        result += &format!(",");
                    }
                }
                result += &format!(")");
                if **body == NullNode {
                    result += &format!(";\n");
                } else {
                    result += &format!("{}", body.to_string(indent));
                }
            }
            ParameterNode(tt, id) => {
                result += &format!(
                    "{} {}",
                    tt.to_string(0).as_str(),
                    id.tk.to_string().as_str()
                );
            }
            ArrayDeclNode(tt, id, args) => {
                result += &format!(
                    "{} {}",
                    tt.to_string(0).as_str(),
                    id.tk.to_string().as_str(),
                );
                for i in 0..args.len() {
                    result += &format!("[{}]", args[i].to_string(0).as_str(),);
                }
            }
        }

        return result;
    }
}
