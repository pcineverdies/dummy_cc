use core::panic;

use crate::lexer::lexer_impl::{Keyword, Tk, Token};

/// AstNode
///
/// Possible nodes from the AST
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum AstNode {
    // ArrayDeclNode: Type of array, identifier, expression for size
    ArrayDeclNode(Box<AstNodeWrapper>, Token, Box<AstNodeWrapper>),
    // BinaryNode: Token of operator, expression left and expression right
    BinaryNode(Token, Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    // CastNode: Type of casting and expression to cast
    CastNode(Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    // CompoundNode: List of statements
    CompoundNode(Vec<AstNodeWrapper>),
    // DeclarationList: List of declarations
    DeclarationList(Vec<AstNodeWrapper>),
    // ExprStatementNode: Expression to be used as statement
    ExprStatementNode(Box<AstNodeWrapper>),
    // ForNode: First, second and third optional expression, body compound statement
    ForNode(Box<AstNodeWrapper>, Box<AstNodeWrapper>, Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    // FuncDeclNode: Return type, identifier, list of arguments and body
    FuncDeclNode(Box<AstNodeWrapper>, Token, Vec<AstNodeWrapper>, Box<AstNodeWrapper>),
    // IfNode: Expression to be used as condition, if block and else (optional) block
    IfNode(Box<AstNodeWrapper>, Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    // JumpNode: Token and expression (present only for return)
    JumpNode(Token, Box<AstNodeWrapper>),
    // ParameterNode: Identifier and type of parameter
    ParameterNode(Token, Box<AstNodeWrapper>),
    // PrefixNode: Prefix operator and following expression
    PrefixNode(Token, Box<AstNodeWrapper>),
    // PrimaryNode: Token of primary node
    PrimaryNode(Token),
    // ProcedureNode: Primary node with the identifier of the funciton and list of arguments
    ProcedureNode(Box<AstNodeWrapper>, Vec<AstNodeWrapper>),
    // SelectorNode: Expression to dereference and expression to be used as selector value
    SelectorNode(Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    // TypeNode: Type to be used
    TypeNode(TypeWrapper),
    // VarDeclNode: Type of the declaration, identifier and (optional) expression
    VarDeclNode(Box<AstNodeWrapper>, Token, Box<AstNodeWrapper>),
    // WhileNode: Conditino of the while, body
    WhileNode(Box<AstNodeWrapper>, Box<AstNodeWrapper>),
    #[default]
    NullNode,
}

/// TypeNative
///
/// Possible list of native types in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TypeNative {
    U32,
    U16,
    U8,
    I32,
    I16,
    I8,
    Void,
    #[default]
    Null,
}

/// TypeWrapper
///
/// Wrapper for a type native in order to inclue all the information related to the type: number of
/// pointers and constant value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeWrapper {
    pub type_native: TypeNative,
    pub pointer: u32,
    pub constant: bool,
}

/// SourceReference
///
/// For each node, we want to store the information about the beginning character and ending
/// character in the source file
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SourceReference {
    pub init_line: u32,
    pub init_char: u32,
    pub last_char: u32,
    pub last_line: u32,
}

/// AstNodeWrapper
///
/// Wrapper fro the AST node, containg additional information about the node (TAstNode)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AstNodeWrapper {
    pub node: AstNode,
    pub source_ref: SourceReference,
    pub type_ref: TypeWrapper,
    pub is_lvalue: bool,
}

use AstNode::*;

impl AstNode {
    /// AstNode::new_declaration_list
    ///
    /// Create a DeclarationList
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_declaration_list(an1: &Vec<AstNodeWrapper>) -> AstNode {
        DeclarationList(an1.clone())
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
    pub fn new_binary(an1: &Token, an2: &AstNodeWrapper, an3: &AstNodeWrapper) -> AstNode {
        BinaryNode(an1.clone(), Box::new(an2.clone()), Box::new(an3.clone()))
    }

    /// AstNode::new_prefix
    ///
    /// Create a PrefixNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_prefix(an1: &Token, an2: &AstNodeWrapper) -> AstNode {
        PrefixNode(an1.clone(), Box::new(an2.clone()))
    }

    /// AstNode::new_cast
    ///
    /// Create a CastNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_cast(an1: &AstNodeWrapper, an2: &AstNodeWrapper) -> AstNode {
        CastNode(Box::new(an1.clone()), Box::new(an2.clone()))
    }

    /// AstNode::new_type
    ///
    /// Create a TypeNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_type(t: &TypeWrapper) -> AstNode {
        TypeNode(t.clone())
    }

    /// AstNode::new_procedure
    ///
    /// Create a ProcedureNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_procedure(an1: &AstNodeWrapper, an2: &Vec<AstNodeWrapper>) -> AstNode {
        ProcedureNode(Box::new(an1.clone()), an2.clone())
    }

    /// AstNode::new_selector
    ///
    /// Create a SelectorNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_selector(an1: &AstNodeWrapper, an2: &AstNodeWrapper) -> AstNode {
        SelectorNode(Box::new(an1.clone()), Box::new(an2.clone()))
    }

    /// AstNode::new_compound
    ///
    /// Create a CompoundNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_compound(an1: &Vec<AstNodeWrapper>) -> AstNode {
        CompoundNode(an1.clone())
    }

    /// AstNode::new_expr_statement
    ///
    /// Create a ExprStatementNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_expr_statement(an1: &AstNodeWrapper) -> AstNode {
        ExprStatementNode(Box::new(an1.clone()))
    }

    /// AstNode::new_if
    ///
    /// Create a IfNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_if(an1: &AstNodeWrapper, an2: &AstNodeWrapper, an3: &AstNodeWrapper) -> AstNode {
        IfNode(Box::new(an1.clone()), Box::new(an2.clone()), Box::new(an3.clone()))
    }

    /// AstNode::new_while
    ///
    /// Create a WhileNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_while(an1: &AstNodeWrapper, an2: &AstNodeWrapper) -> AstNode {
        WhileNode(Box::new(an1.clone()), Box::new(an2.clone()))
    }

    /// AstNode::new_for
    ///
    /// Create a ForNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_for(an1: &AstNodeWrapper, an2: &AstNodeWrapper, an3: &AstNodeWrapper, an4: &AstNodeWrapper) -> AstNode {
        ForNode(Box::new(an1.clone()), Box::new(an2.clone()), Box::new(an3.clone()), Box::new(an4.clone()))
    }

    /// AstNode::new_jump
    ///
    /// Create a JumpNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_jump(an1: &Token, an2: &AstNodeWrapper) -> AstNode {
        JumpNode(an1.clone(), Box::new(an2.clone()))
    }

    /// AstNode::new_parameter
    ///
    /// Create a ParameterNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_parameter(an1: &Token, an2: &AstNodeWrapper) -> AstNode {
        ParameterNode(an1.clone(), Box::new(an2.clone()))
    }

    /// AstNode::new_var_decl
    ///
    /// Create a VarDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_var_decl(an1: &AstNodeWrapper, an2: &Token, an3: &AstNodeWrapper) -> AstNode {
        VarDeclNode(Box::new(an1.clone()), an2.clone(), Box::new(an3.clone()))
    }

    /// AstNode::new_func_decl
    ///
    /// Create a FuncDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_func_decl(an1: &AstNodeWrapper, an2: &Token, an3: &Vec<AstNodeWrapper>, an4: &AstNodeWrapper) -> AstNode {
        FuncDeclNode(Box::new(an1.clone()), an2.clone(), an3.clone(), Box::new(an4.clone()))
    }

    /// AstNode::new_array_decl
    ///
    /// Create a ArrayDeclNode
    ///
    /// @in [...] What is necessary to build the node
    /// @return [AstNode] Built node
    pub fn new_array_decl(an1: &AstNodeWrapper, an2: &Token, an3: &AstNodeWrapper) -> AstNode {
        ArrayDeclNode(Box::new(an1.clone()), an2.clone(), Box::new(an3.clone()))
    }
}

impl AstNodeWrapper {
    /// AstNodeWrapper::get_indent
    ///
    /// Produce a string with the correct number of spaces with respect to the required indentation
    ///
    /// @in indent[u32] How much to indent
    /// @return [String] Result of the identation
    fn get_indent(&self, indent: u32) -> String {
        let mut result = String::from("");
        for _ in 0..indent {
            result += "  ";
        }
        result
    }

    /// AstNodeWrapper::to_string
    ///
    /// Transform the current AstNode to a string, exploiting the function in a recursive fashion.
    /// The parameter indent is used to indicate how much to indent, so that the final result is
    /// consistent with a readable formato of the code
    ///
    /// @in indent[u32] How much to indent
    /// @return [String] Result of the string conversion
    pub fn to_string(&self, indent: u32) -> String {
        let mut result = String::from("");
        match self.node {
            AstNode::CompoundNode(_) => {}
            _ => result += &self.get_indent(indent),
        }
        match &self.node {
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
                result += &format!("({}{})", tk.tk.to_string().as_str(), expr.to_string(0).as_str());
            }
            CastNode(cn, expr) => {
                result += &format!("(({}){})", cn.to_string(0).as_str(), expr.to_string(0).as_str());
            }
            TypeNode(t) => {
                result += &format!("{}", t.to_string());
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
                result += &format!("(({})", expr.to_string(0).as_str(),);
                result += &format!("[{}])", args.to_string(0).as_str(),);
            }
            CompoundNode(value) => {
                if value.len() == 0 {
                    result += "{}\n";
                } else {
                    result += "{\n";
                    for s in value {
                        result += &s.to_string(indent + 1);
                    }
                    result += &self.get_indent(indent);
                    result += "}\n";
                }
            }
            ExprStatementNode(expr) => {
                if expr.node != NullNode {
                    result += &format!("{}", expr.to_string(0).as_str(),);
                }
                result += ";\n";
            }
            IfNode(expr, statements_if, statements_else) => {
                result += &format!("if({}){}", &expr.to_string(0).as_str(), &statements_if.to_string(indent).as_str());
                let else_print = statements_else.to_string(indent);
                if else_print.len() as u32 > (indent as u32) * 2 {
                    result += &self.get_indent(indent);
                    result += &format!("else{}", else_print);
                }
            }
            WhileNode(expr, statements) => {
                result += &format!("while({}){}", &expr.to_string(0).as_str(), &statements.to_string(indent).as_str());
            }
            ForNode(decl, expr, ass, statements) => {
                result += &format!(
                    "for({}; {}; {}){}",
                    &decl.to_string(0).as_str(),
                    &expr.to_string(0).as_str(),
                    &ass.to_string(0).as_str(),
                    &statements.to_string(indent).as_str()
                );
            }
            JumpNode(kw, expr) => match expr.node {
                NullNode => {
                    result += &format!("{};\n", kw.tk.to_string().as_str(),);
                }
                _ => {
                    result += &format!("{} {};\n", kw.tk.to_string().as_str(), expr.to_string(0).as_str());
                }
            },
            VarDeclNode(tt, id, expr) => {
                result += &format!("{} {}", tt.to_string(0).as_str(), id.tk.to_string().as_str());
                if expr.node == NullNode {
                    result += &format!(";\n");
                } else {
                    result += &format!(" = {};\n", expr.to_string(0).as_str(),);
                }
            }
            FuncDeclNode(tt, id, args, body) => {
                result += &format!("{} {}(", tt.to_string(0).as_str(), id.tk.to_string().as_str());
                for i in 0..args.len() {
                    result += &format!("{}", args[i].to_string(0).as_str(),);
                    if i != args.len() - 1 {
                        result += &format!(",");
                    }
                }
                result += &format!(")");
                if body.node == NullNode {
                    result += &format!(";\n");
                } else {
                    result += &format!("{}", body.to_string(indent));
                }
            }
            ParameterNode(id, tt) => {
                result += &format!("{} {}", tt.to_string(0).as_str(), id.tk.to_string().as_str());
            }
            ArrayDeclNode(tt, id, arg) => {
                result += &format!("{} {}", tt.to_string(0).as_str(), id.tk.to_string().as_str(),);
                result += &format!("[{}];\n", arg.to_string(0).as_str(),);
            }
            DeclarationList(list) => {
                for l in list {
                    result += &format!("{}", l.to_string(indent).as_str())
                }
            }
        }

        return result;
    }
}

impl SourceReference {
    /// AstNodeWrapper::from_token
    ///
    /// Create a source file reference object starting from a token
    ///
    /// @in tk[&Token] Token to use to get the source file reference
    /// @return [SourceReference] Result of creation
    pub fn from_token(tk: &Token) -> SourceReference {
        SourceReference {
            last_line: tk.line_number,
            init_line: tk.line_number,
            last_char: tk.last_character,
            init_char: tk.first_character,
        }
    }

    /// AstNodeWrapper::merge
    ///
    /// Create a source file reference object mergint two of them
    ///
    /// @in sr1[&SourceReference] First source reference object
    /// @in sr1[&SourceReference] Second source reference object
    /// @return [SourceReference] Result of creation
    pub fn merge(sr1: &SourceReference, sr2: &SourceReference) -> SourceReference {
        let mut result = SourceReference { ..Default::default() };
        // The first source reference object starts before the second one
        if sr1.init_line < sr2.init_line {
            result.init_line = sr1.init_line;
            result.init_char = sr1.init_char;
            result.last_line = sr2.last_line;
            result.last_char = sr2.last_char;
        // The second source reference object starts before the first one
        } else if sr1.init_line > sr2.init_line {
            result.init_line = sr2.init_line;
            result.init_char = sr2.init_char;
            result.last_line = sr1.last_line;
            result.last_char = sr1.last_char;
        } else {
            // The first source reference object starts before the second one
            if sr1.init_char < sr2.init_char {
                result.init_line = sr1.init_line;
                result.init_char = sr1.init_char;
                result.last_line = sr2.last_line;
                result.last_char = sr2.last_char;
            // The second source reference object starts before the first one
            } else {
                result.init_line = sr2.init_line;
                result.init_char = sr2.init_char;
                result.last_line = sr1.last_line;
                result.last_char = sr1.last_char;
            }
        }

        result
    }
}

impl TypeNative {
    /// TypeNative::from_token
    ///
    /// Create a type native object starting from a valid token
    ///
    /// @in tk[&Token] Token to use
    /// @return [TypeNative] Result
    pub fn from_token(tk: &Token) -> TypeNative {
        match &tk.tk {
            Tk::Keyword(k) => match k {
                Keyword::U8 => return TypeNative::U8,
                Keyword::U16 => return TypeNative::U16,
                Keyword::U32 => return TypeNative::U32,
                Keyword::I8 => return TypeNative::I8,
                Keyword::I16 => return TypeNative::I16,
                Keyword::I32 => return TypeNative::I32,
                Keyword::Void => return TypeNative::Void,
                _ => panic!("Cannot create type from non-type keyword"),
            },
            _ => {
                panic!("Cannot create type from token which is not Keyword")
            }
        }
    }

    /// TypeNative::get_size
    ///
    /// Get size in bytes of a give type
    ///
    /// @return [u32] size
    pub fn get_size(&self) -> u32 {
        match &self {
            TypeNative::U8 | TypeNative::I8 => 1,
            TypeNative::U16 | TypeNative::I16 => 2,
            TypeNative::U32 | TypeNative::I32 => 4,
            _ => panic!("Cannot get size of non-sized type"),
        }
    }

    /// TypeNative::is_signed
    ///
    /// Is the type signed or not
    ///
    /// @return [bool] signed
    pub fn is_signed(&self) -> bool {
        match &self {
            TypeNative::I8 | TypeNative::I16 | TypeNative::I32 => return true,
            _ => return false,
        }
    }
}

impl TypeWrapper {
    /// TypeWrapper::to_string
    ///
    /// Print the type with the const keyword, type and pointers
    ///
    /// @return [String] Result
    pub fn to_string(&self) -> String {
        let mut result = String::from("");

        if self.constant {
            result += "const ";
        }

        match &self.type_native {
            TypeNative::U32 => result += "u32",
            TypeNative::U16 => result += "u16",
            TypeNative::U8 => result += "u8",
            TypeNative::I32 => result += "i32",
            TypeNative::I16 => result += "i16",
            TypeNative::I8 => result += "i8",
            TypeNative::Void => result += "void",
            TypeNative::Null => result += "null",
        };

        for _ in 0..self.pointer {
            result += "*";
        }

        result
    }

    /// TypeWrapper::are_compatible
    ///
    /// Check whether two types are compatible or not. This function can be expanded in order to
    /// allow automatic casting
    ///
    /// @in a [&TypeWrapper]: first type
    /// @in b [&TypeWrapper]: second type
    /// @return [bool] result of the comparison
    pub fn are_compatible(a: &TypeWrapper, b: &TypeWrapper) -> bool {
        // Both type and number of pointers must be identical
        if a.pointer != b.pointer || a.type_native != b.type_native {
            return false;
        }
        return true;
    }
}

impl Default for TypeWrapper {
    /// TypeWrapper::default
    ///
    /// Creates a new default TypeWrapper
    fn default() -> TypeWrapper {
        TypeWrapper {
            constant: false,
            type_native: TypeNative::Null,
            pointer: 0,
        }
    }
}
