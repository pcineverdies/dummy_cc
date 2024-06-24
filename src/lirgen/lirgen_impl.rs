use crate::ast::ast_impl::{AstNode, AstNodeWrapper};
use crate::lexer::lexer_impl::{Keyword, Operator, Tk, Token};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeBranch {
    Always,
    Set,
    NotSet,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrNode {
    Binary(Operator, u32, u32, u32, bool), // Token, dest, src1, src2, signed
    Unary(Operator, u32, u32),             // Token, dest, src
    Call(String, Vec<u32>, u32),           // Function, operands, return
    LoadR(u32, u32, u32),                  // dest, source with address, size
    LoadA(u32, String),                    // dest, address to store
    MovR(u32, u32),                        // dest, src
    MovC(u32, u32),                        // dest, constant
    Return(u32),                           // val
    StoreL(String, u32, u32),              // address, source, size
    StoreR(u32, u32, u32),                 // dest, source, size
    Branch(TypeBranch, u32, String),       // Type of branch, register, label
    FunctionStart,
    Label(String),                        // Label
    Cast(u32, u32, u32, u32, bool, bool), // dst, src, size_dest, size_src, signed_dst, signed_src
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LirgenResult {
    pub ir_list: Vec<IrNode>,
    pub result_register: u32,
}

pub struct Lirgen {
    pub current_register: u32,
}

use AstNode::*;
use IrNode::*;

impl Lirgen {
    pub fn new() -> Lirgen {
        return Lirgen { current_register: 0 };
    }

    fn get_register(&mut self) -> u32 {
        self.current_register += 1;
        return self.current_register;
    }

    pub fn linearize_ast(&mut self, ast: &AstNodeWrapper) -> Vec<IrNode> {
        let result = self.linearize(ast, false);
        return result.ir_list;
    }

    fn linearize(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        match ast.node {
            DeclarationList(_) => return self.linearize_declaration_list(ast, get_address),
            VarDeclNode(_, _, _) => return self.linearize_var_decl_node(ast, get_address),
            PrimaryNode(_) => return self.linearize_primary_node(ast, get_address),
            BinaryNode(_, _, _) => return self.linearize_binary_node(ast, get_address),
            CastNode(_, _) => return self.linearize_cast_node(ast, get_address),
            FuncDeclNode(_, _, _, _) => return self.linearize_func_decl_node(ast, get_address),
            CompoundNode(_) => return self.linearize_compound_node(ast, get_address),
            ExprStatementNode(_) => return self.linearize_expr_statement_node(ast, get_address),
            JumpNode(_, _) => return self.linearize_jump_node(ast, get_address),
            TypeNode(_) => panic!("TypeNode cannot be linearized!"),
            NullNode => panic!("NullNode cannot be linearized!"),
            // ArrayDeclNode(_, _, _) => {}
            // ForNode(_, _, _, _) => {}
            // IfNode(_, _, _) => {}
            // ParameterNode(_, _) => {}
            // PrefixNode(_, _) => {}
            // ProcedureNode(_, _) => {}
            // SelectorNode(_, _) => {}
            // WhileNode(_, _) => {}
            _ => todo!(),
        }
    }

    fn linearize_jump_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::JumpNode(token, expr) = &ast.node {
            match token.tk {
                Tk::Keyword(Keyword::Return) => {
                    let mut result = LirgenResult { ..Default::default() };
                    if expr.node != AstNode::NullNode {
                        let mut e_lin = self.linearize(expr, get_address);
                        result.ir_list.append(&mut e_lin.ir_list);
                        result.ir_list.push(IrNode::Return(e_lin.result_register));
                    } else {
                        result.ir_list.push(IrNode::Return(0));
                    }

                    return result;
                }
                _ => todo!(),
            }
        }

        panic!("AstNode is not of type ExprStatementNode");
    }

    fn linearize_expr_statement_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::ExprStatementNode(expr) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            let mut e_lin = self.linearize(expr, get_address);
            result.ir_list.append(&mut e_lin.ir_list);

            return result;
        }

        panic!("AstNode is not of type ExprStatementNode");
    }

    fn linearize_compound_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::CompoundNode(list) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };

            for l in list {
                let mut l_lin = self.linearize(l, get_address);
                result.ir_list.append(&mut l_lin.ir_list);
            }

            return result;
        }

        panic!("AstNode is not of type CompoundNode");
    }

    fn linearize_declaration_list(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        let mut result = LirgenResult { ..Default::default() };

        if let AstNode::DeclarationList(list) = &ast.node {
            for elem in list {
                let mut lin = self.linearize(&elem, get_address);
                result.ir_list.append(&mut lin.ir_list);
            }
            return result;
        }

        panic!("AstNode is not of type DeclarationList");
    }

    fn linearize_var_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::VarDeclNode(tt, name, expression) = &ast.node {
            let mut expression_lin = self.linearize(&expression, get_address);
            let store_node = StoreL(
                Lirgen::get_identifier(&name),
                expression_lin.result_register,
                tt.type_ref.type_native.get_size(),
            );
            expression_lin.ir_list.push(store_node);
            return expression_lin;
        }

        panic!("AstNode is not of type VarDeclNode");
    }

    fn linearize_primary_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::PrimaryNode(token) = &ast.node {
            match &token.tk {
                Tk::Identifier(id) => {
                    let load_register = self.get_register();
                    let load_address = LoadA(load_register, id.to_string());
                    if get_address {
                        return LirgenResult {
                            ir_list: vec![load_address],
                            result_register: load_register,
                        };
                    }
                    let result_register = self.get_register();
                    let load_value = LoadR(result_register, load_register, ast.type_ref.type_native.get_size());
                    return LirgenResult {
                        ir_list: vec![load_address, load_value],
                        result_register,
                    };
                }
                Tk::IntegerLiteral(num) => {
                    let result_register = self.get_register();
                    let store_constant_node = MovC(result_register, *num as u32);
                    return LirgenResult {
                        ir_list: vec![store_constant_node],
                        result_register,
                    };
                }
                Tk::Char(c) => {
                    let result_register = self.get_register();
                    let store_constant_node = MovC(result_register, *c as u32);
                    return LirgenResult {
                        ir_list: vec![store_constant_node],
                        result_register,
                    };
                }
                _ => panic!("Token cannot be handled as PrimaryNode"),
            }
        }

        panic!("AstNode is not of type PrimaryNode");
    }

    fn linearize_binary_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::BinaryNode(token, exp1, exp2) = &ast.node {
            let mut exp1_lin = self.linearize(&exp1, get_address);
            let mut exp2_lin = self.linearize(&exp2, get_address);
            let is_signed = ast.type_ref.type_native.is_signed();
            let operator = Lirgen::get_operator(&token);

            let new_binary = IrNode::Binary(
                operator,
                self.get_register(),
                exp1_lin.result_register,
                exp2_lin.result_register,
                is_signed,
            );
            let mut ir_list: Vec<IrNode> = vec![];

            ir_list.append(&mut exp1_lin.ir_list);
            ir_list.append(&mut exp2_lin.ir_list);
            ir_list.push(new_binary);

            return LirgenResult {
                ir_list,
                result_register: self.current_register,
            };
        }

        panic!("AstNode is not of type BinaryNode");
    }

    fn linearize_cast_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::CastNode(dest_type, exp) = &ast.node {
            let mut exp_lin = self.linearize(&exp, get_address);

            if dest_type.type_ref.pointer > 0 {
                return exp_lin;
            }

            let new_cast = IrNode::Cast(
                self.get_register(),
                exp_lin.result_register,
                dest_type.type_ref.type_native.get_size(),
                exp.type_ref.type_native.get_size(),
                dest_type.type_ref.type_native.is_signed(),
                exp.type_ref.type_native.is_signed(),
            );
            let mut ir_list: Vec<IrNode> = vec![];

            ir_list.append(&mut exp_lin.ir_list);
            ir_list.push(new_cast);

            return LirgenResult {
                ir_list,
                result_register: self.current_register,
            };
        }

        panic!("AstNode is not of type CastNode");
    }

    fn linearize_func_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool) -> LirgenResult {
        if let AstNode::FuncDeclNode(_, name, _, body) = &ast.node {
            let mut body_lin = self.linearize(body, get_address);

            let mut ir_list: Vec<IrNode> = vec![];

            ir_list.push(IrNode::Label(Lirgen::get_identifier(name)));
            ir_list.push(IrNode::FunctionStart);
            ir_list.append(&mut body_lin.ir_list);

            return LirgenResult {
                ir_list,
                result_register: self.current_register,
            };
        }

        panic!("AstNode is not of type FuncDeclNode");
    }
    fn get_identifier(token: &Token) -> String {
        if let Tk::Identifier(s) = &token.tk {
            return s.clone();
        }
        panic!("Cannot extract identifier from non-identifier token: {:#?}", token);
    }

    fn get_operator(token: &Token) -> Operator {
        if let Tk::Operator(o) = &token.tk {
            return o.clone();
        }
        panic!("Cannot extract operator from non-identifier token: {:#?}", token);
    }

    pub fn to_string(ir_list: &Vec<IrNode>) -> String {
        let mut result = "".to_string();
        for l in ir_list {
            result += &l.to_string();
        }

        return result;
    }
}

impl IrNode {
    pub fn to_string(&self) -> String {
        match &self {
            MovC(d, c) => return format!("  mov v{}, ${}\n", d, c),
            MovR(d, s) => return format!("  mov v{}, v{}\n", d, s),
            StoreL(s, src, size) => return format!("  store.{} @{}, v{}\n", IrNode::get_size_name(*size), s, src),
            StoreR(s, src, size) => return format!("  store.{} v{}, v{}\n", IrNode::get_size_name(*size), s, src),
            LoadR(s, src, size) => return format!("  load.{} v{}, v{}\n", IrNode::get_size_name(*size), s, src),
            LoadA(d, s) => return format!("  ld v{}, @{}\n", d, s),
            Branch(tt, r, s) => {
                let mut result = format!("  ");
                match tt {
                    TypeBranch::Set => result += &format!("b.set v{}", r),
                    TypeBranch::NotSet => result += &format!("b.nset v{}", r),
                    TypeBranch::Always => result += &format!("b"),
                }

                result += &format!("<{}>\n", s);

                return result;
            }
            Label(s) => return format!("\n<{}>:\n", s),
            Call(s, ops, r) => {
                let mut result = format!("  v{} <- call <{}> [", r, s);

                for o in ops {
                    result += &format!("v{}, ", o);
                }

                result += "]\n";

                return result;
            }
            Binary(tk, dest, src1, src2, signed) => {
                let mut result = "  ".to_string();

                match tk {
                    Operator::EqualCompare => result += "seq",
                    Operator::DiffCompare => result += "sneq",
                    Operator::LTCompare => result += "slt",
                    Operator::GTCompare => result += "sgt",
                    Operator::LECompare => result += "sle",
                    Operator::GECompare => result += "sge",
                    Operator::Minus => result += "sub",
                    Operator::Plus => result += "add",
                    Operator::Asterisk => result += "mul",
                    Operator::Slash => result += "div",
                    Operator::XorOp => result += "xor",
                    Operator::AndOp => result += "and",
                    Operator::OrOp => result += "or",
                    Operator::Module => result += "rem",
                    Operator::LShift => result += "sl",
                    Operator::RShift => result += "sr",
                    _ => panic!("Invalid binary operator {:#?}", tk),
                }

                if *signed {
                    result += &format!(".s");
                }

                result += &format!(" v{}, v{}, v{}\n", dest, src1, src2);

                return result;
            }
            Cast(dst, src, s_d, s_s, sign_d, sign_s) => {
                let mut result = "  ".to_string();
                let size_dest = s_d * 8;
                let size_src = s_s * 8;
                let sign_dest = if *sign_d { "i" } else { "u" };
                let sign_src = if *sign_s { "i" } else { "u" };

                result += &format!("cast.{}{}.{}{} v{}, v{}\n", sign_dest, size_dest, sign_src, size_src, *dst, *src);

                return result;
            }
            FunctionStart => return String::from("  //! Prologue\n"),
            Return(r) => return format!("  ret v{r}\n  //! Epilogue\n"),
            _ => todo!(),
        }
    }

    fn get_size_name(size: u32) -> String {
        if size == 1 {
            return String::from("b");
        } else if size == 2 {
            return String::from("h");
        } else if size == 4 {
            return String::from("w");
        }

        panic!("Unexpected size {}", size);
    }
}
