use crate::ast::ast_impl::{AstNode, AstNodeWrapper, TypeWrapper};
use crate::lexer::lexer_impl::{Keyword, Operator, Tk, Token};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompareType {
    Always,
    GT,
    GE,
    LT,
    LE,
    EQ,
    NE,
    S,
    NS,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrNode {
    // List of nodes
    Program(Vec<IrNode>),
    // name of the function, return type, argument types, list of nodes
    FunctionDeclaration(String, TypeWrapper, Vec<TypeWrapper>, Vec<IrNode>),
    // type of the allocated data, destination register, source register, is global, size in bytes,
    // from register
    Alloc(TypeWrapper, u32, u32, bool, u32, bool),
    // type of the returned value, source register
    Return(TypeWrapper, u32),
    // type of the data, destination register, constant value
    MovC(TypeWrapper, u32, u32),
    // destination type, source type, destination register, source register
    Cast(TypeWrapper, TypeWrapper, u32, u32),
    // type of the allocated data, destination address register, source register
    Store(TypeWrapper, u32, u32),
    // type of the allocated data, destination register, source label
    LoadA(TypeWrapper, u32, String),
    // type of the allocated data, destination register, source address register
    LoadR(TypeWrapper, u32, u32),
    // label
    Label(u32),
    // name of the function, return type, register arguments, return register
    Call(String, TypeWrapper, Vec<u32>, u32),
    // compare operation to use, type to use, source1, source2, label to jump to
    Branch(CompareType, TypeWrapper, u32, u32, u32),
    // operator, type, destination, source1, source2
    Binary(Operator, TypeWrapper, u32, u32, u32),
    // operator, type, destination, source
    Unary(TypeWrapper, Operator, u32, u32),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LirgenResult {
    pub ir_list: Vec<IrNode>,
    pub result_register: u32,
}

pub struct Lirgen {
    current_register: u32,
    current_label: u32,
    variable_pointers: Vec<(String, u32)>,
}

use AstNode::*;
use IrNode::*;

impl Lirgen {
    /// Lirgen::new
    ///
    /// Create a new empty Linear IR generator
    pub fn new() -> Lirgen {
        return Lirgen {
            current_register: 0,
            current_label: 0,
            variable_pointers: vec![],
        };
    }

    fn get_pointer_variable(&self, s: &String) -> Option<u32> {
        for elem in &self.variable_pointers {
            if *elem.0 == *s {
                return Some(elem.1);
            }
        }
        return None;
    }

    fn add_pointer_variable(&mut self, s: &String, r: u32) {
        self.variable_pointers.push((s.clone(), r));
    }

    fn erase_registers(&mut self) {
        self.current_register = 0;
        self.current_label = 0;
        self.variable_pointers.clear();
    }

    fn get_register(&mut self) -> u32 {
        self.current_register += 1;
        return self.current_register;
    }

    fn get_label(&mut self) -> u32 {
        self.current_label += 1;
        return self.current_label;
    }

    pub fn linearize_ast(&mut self, ast: &AstNodeWrapper) -> IrNode {
        IrNode::Program(self.linearize(ast, false, 0, 0).ir_list)
    }

    fn linearize(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        match ast.node {
            DeclarationList(..) => return self.linearize_declaration_list(ast, get_address, break_dest, continue_dest),
            VarDeclNode(..) => return self.linearize_var_decl_node(ast, get_address, break_dest, continue_dest),
            PrimaryNode(..) => return self.linearize_primary_node(ast, get_address, break_dest, continue_dest),
            JumpNode(..) => return self.linearize_jump_node(ast, get_address, break_dest, continue_dest),
            CompoundNode(..) => return self.linearize_compound_node(ast, get_address, break_dest, continue_dest),
            FuncDeclNode(..) => return self.linearize_func_decl_node(ast, get_address, break_dest, continue_dest),
            BinaryNode(..) => return self.linearize_binary_node(ast, get_address, break_dest, continue_dest),
            CastNode(..) => return self.linearize_cast_node(ast, get_address, break_dest, continue_dest),
            ExprStatementNode(..) => return self.linearize_expr_statement_node(ast, get_address, break_dest, continue_dest),
            ArrayDeclNode(..) => return self.linearize_array_decl_node(ast, get_address, break_dest, continue_dest),
            ProcedureNode(..) => return self.linearize_procedure_node(ast, get_address, break_dest, continue_dest),
            PrefixNode(..) => return self.linearize_prefix_node(ast, get_address, break_dest, continue_dest),
            SelectorNode(..) => return self.linearize_selector_node(ast, get_address, break_dest, continue_dest),
            IfNode(..) => return self.linearize_if_node(ast, get_address, break_dest, continue_dest),
            WhileNode(..) => return self.linearize_while_node(ast, get_address, break_dest, continue_dest),
            ForNode(..) => return self.linearize_for_node(ast, get_address, break_dest, continue_dest),
            TypeNode(..) => panic!("TypeNode cannot be linearized!"),
            NullNode => panic!("NullNode cannot be linearized!"),
            ParameterNode(..) => panic!("ParameterNode cannot be linearized!"),
        }
    }

    /// Lirgen::linearize_selector_node
    ///
    /// Linearize a node of type SelectorNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_selector_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::SelectorNode(left, right) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            let mut l_lin = self.linearize(&left, true, break_dest, continue_dest);
            let mut r_lin = self.linearize(&right, false, break_dest, continue_dest);
            let mut tt = ast.type_ref.clone();
            tt.pointer += 1;

            result.ir_list.append(&mut l_lin.ir_list);
            result.ir_list.append(&mut r_lin.ir_list);

            let size = ast.type_ref.get_size();

            let constant_register = self.get_register();
            let store_constant_node = MovC(tt.clone(), constant_register, size);
            result.ir_list.push(store_constant_node);

            let offset_register = self.get_register();
            let new_op = Binary(Operator::LShift, tt.clone(), offset_register, r_lin.result_register, constant_register);
            result.ir_list.push(new_op);

            let sum_register = self.get_register();
            let new_op = Binary(Operator::Plus, tt.clone(), sum_register, l_lin.result_register, offset_register);
            result.ir_list.push(new_op);
            result.result_register = sum_register;

            if !get_address {
                let result_register = self.get_register();
                let load_value = LoadR(ast.type_ref.clone(), result_register, result.result_register);
                result.result_register = result_register;
                result.ir_list.push(load_value);
            }

            return result;
        }
        panic!("AstNode is not of type SelectorNode");
    }

    /// Lirgen::linearize_prefix_node
    ///
    /// Linearize a node of type PrefixNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_prefix_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::PrefixNode(token, expr) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            match &token.tk {
                Tk::Operator(op) => {
                    if *op == Operator::Not || *op == Operator::Plus || *op == Operator::Minus || *op == Operator::Complement {
                        let mut exp_lin = self.linearize(&expr, false, break_dest, continue_dest);
                        let result_register = self.get_register();
                        result.ir_list.append(&mut exp_lin.ir_list);
                        result
                            .ir_list
                            .push(IrNode::Unary(ast.type_ref.clone(), op.clone(), result_register, exp_lin.result_register));
                        result.result_register = result_register;
                        return result;
                    } else if *op == Operator::Asterisk {
                        let mut exp_lin = self.linearize(&expr, true, break_dest, continue_dest);
                        result.ir_list.append(&mut exp_lin.ir_list);
                        let result_register = self.get_register();
                        let mut tt = ast.type_ref.clone();
                        tt.pointer += 1;
                        let load_value = LoadR(tt, result_register, exp_lin.result_register);
                        result.ir_list.push(load_value);
                        result.result_register = result_register;
                        if !get_address {
                            let result_register = self.get_register();
                            let load_value = LoadR(ast.type_ref.clone(), result_register, result.result_register);
                            result.result_register = result_register;
                            result.ir_list.push(load_value);
                            result.result_register = result_register;
                        }
                        return result;
                    } else if *op == Operator::AndOp {
                        return self.linearize(&expr, true, break_dest, continue_dest);
                    }
                }
                _ => {}
            }
            panic!("Invalid token {} in PrefixNode", token.tk);
        }
        panic!("AstNode is not of type PrefixNode");
    }

    /// Lirgen::linearize_array_decl_node
    ///
    /// Linearize a node of type ArrayDeclNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_array_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::ArrayDeclNode(tt, name, expression) = &ast.node {
            let mut result: LirgenResult = Default::default();
            let mut expression_lin = self.linearize(&expression, get_address, break_dest, continue_dest);
            let init_register = expression_lin.result_register;
            let result_register = self.get_register();
            let store_node = Alloc(tt.type_ref.clone(), result_register, 0, false, init_register, true);

            result.ir_list.append(&mut expression_lin.ir_list);
            result.ir_list.push(store_node);
            result.result_register = result_register;

            self.add_pointer_variable(&Lirgen::get_identifier(name), result_register);
            return result;
        }
        panic!("AstNode is not of type ArrayDeclNode");
    }

    /// Lirgen::linearize_for_node
    ///
    /// Linearize a node of type ForNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_for_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::ForNode(expr1, expr2, expr3, body) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };

            let for_label = self.get_label();
            let for_start_label = self.get_label();
            let for_next_label = self.get_label();
            let for_end_label = self.get_label();

            result.ir_list.push(IrNode::Label(for_label.clone()));
            let mut expr1_lin = self.linearize(expr1, get_address, break_dest, continue_dest);
            result.ir_list.append(&mut expr1_lin.ir_list);
            result.ir_list.push(IrNode::Label(for_start_label.clone()));

            let mut found_compare = false;
            if let BinaryNode(tk, exp1, exp2) = &expr2.node {
                if tk.tk == Tk::Operator(Operator::GECompare)
                    || tk.tk == Tk::Operator(Operator::GTCompare)
                    || tk.tk == Tk::Operator(Operator::LECompare)
                    || tk.tk == Tk::Operator(Operator::LTCompare)
                    || tk.tk == Tk::Operator(Operator::EqualCompare)
                    || tk.tk == Tk::Operator(Operator::DiffCompare)
                {
                    found_compare = true;
                    let mut expr1_lin = self.linearize(exp1, get_address, break_dest, continue_dest);
                    let mut expr2_lin = self.linearize(exp2, get_address, break_dest, continue_dest);
                    result.ir_list.append(&mut expr1_lin.ir_list);
                    result.ir_list.append(&mut expr2_lin.ir_list);
                    result.ir_list.push(IrNode::Branch(
                        CompareType::from_token(&tk).opposite(),
                        exp1.type_ref.clone(),
                        expr1_lin.result_register,
                        expr2_lin.result_register,
                        for_end_label,
                    ));
                }
            }

            if !found_compare {
                let mut expr_lin = self.linearize(expr2, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut expr_lin.ir_list);
                result.ir_list.push(IrNode::Branch(
                    CompareType::NS,
                    expr2.type_ref.clone(),
                    expr_lin.result_register,
                    0,
                    for_end_label,
                ));
            }

            let mut body_lin = self.linearize(body, get_address, for_end_label, for_next_label);
            result.ir_list.append(&mut body_lin.ir_list);
            result.ir_list.push(IrNode::Label(for_next_label.clone()));

            let mut expr3_lin = self.linearize(expr3, get_address, break_dest, continue_dest);
            result.ir_list.append(&mut expr3_lin.ir_list);
            result
                .ir_list
                .push(IrNode::Branch(CompareType::Always, ast.type_ref.clone(), 0, 0, for_start_label));
            result.ir_list.push(IrNode::Label(for_end_label.clone()));

            return result;
        }
        panic!("AstNode is not of type WhileNode");
    }

    /// Lirgen::linearize_while_node
    ///
    /// Linearize a node of type WhileNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_while_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::WhileNode(expr, body) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            let while_label = self.get_label();
            let while_end_label = self.get_label();
            result.ir_list.push(IrNode::Label(while_label));

            let mut found_compare = false;
            if let BinaryNode(tk, exp1, exp2) = &expr.node {
                if tk.tk == Tk::Operator(Operator::GECompare)
                    || tk.tk == Tk::Operator(Operator::GTCompare)
                    || tk.tk == Tk::Operator(Operator::LECompare)
                    || tk.tk == Tk::Operator(Operator::LTCompare)
                    || tk.tk == Tk::Operator(Operator::EqualCompare)
                    || tk.tk == Tk::Operator(Operator::DiffCompare)
                {
                    found_compare = true;
                    let mut expr1_lin = self.linearize(exp1, get_address, break_dest, continue_dest);
                    let mut expr2_lin = self.linearize(exp2, get_address, break_dest, continue_dest);
                    result.ir_list.append(&mut expr1_lin.ir_list);
                    result.ir_list.append(&mut expr2_lin.ir_list);
                    result.ir_list.push(IrNode::Branch(
                        CompareType::from_token(&tk).opposite(),
                        exp1.type_ref.clone(),
                        expr1_lin.result_register,
                        expr2_lin.result_register,
                        while_end_label,
                    ));
                }
            }

            if !found_compare {
                let mut expr_lin = self.linearize(expr, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut expr_lin.ir_list);
                result.ir_list.push(IrNode::Branch(
                    CompareType::NS,
                    expr.type_ref.clone(),
                    expr_lin.result_register,
                    0,
                    while_end_label,
                ));
            }

            let mut body_lin = self.linearize(body, get_address, while_end_label, while_label);
            result.ir_list.append(&mut body_lin.ir_list);
            result
                .ir_list
                .push(IrNode::Branch(CompareType::Always, ast.type_ref.clone(), 0, 0, while_label));
            result.ir_list.push(IrNode::Label(while_end_label));

            return result;
        }
        panic!("AstNode is not of type WhileNode");
    }

    /// Lirgen::linearize_if_node
    ///
    /// Linearize a node of type IfNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_if_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::IfNode(expr, body, else_body) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };

            let if_next_label = self.get_label();
            let if_end_label = self.get_label();

            let if_else_label = if else_body.node != AstNode::NullNode {
                self.get_label()
            } else {
                if_end_label
            };

            let mut found_compare = false;
            if let BinaryNode(tk, exp1, exp2) = &expr.node {
                if tk.tk == Tk::Operator(Operator::GECompare)
                    || tk.tk == Tk::Operator(Operator::GTCompare)
                    || tk.tk == Tk::Operator(Operator::LECompare)
                    || tk.tk == Tk::Operator(Operator::LTCompare)
                    || tk.tk == Tk::Operator(Operator::EqualCompare)
                    || tk.tk == Tk::Operator(Operator::DiffCompare)
                {
                    found_compare = true;
                    let mut expr1_lin = self.linearize(exp1, get_address, break_dest, continue_dest);
                    let mut expr2_lin = self.linearize(exp2, get_address, break_dest, continue_dest);
                    result.ir_list.append(&mut expr1_lin.ir_list);
                    result.ir_list.append(&mut expr2_lin.ir_list);
                    result.ir_list.push(IrNode::Branch(
                        CompareType::from_token(&tk).opposite(),
                        exp1.type_ref.clone(),
                        expr1_lin.result_register,
                        expr2_lin.result_register,
                        if_else_label,
                    ));
                }
            }

            if !found_compare {
                let mut expr_lin = self.linearize(expr, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut expr_lin.ir_list);
                result.ir_list.push(IrNode::Branch(
                    CompareType::NS,
                    expr.type_ref.clone(),
                    expr_lin.result_register,
                    0,
                    if_else_label,
                ));
            }

            let mut body_lin = self.linearize(body, get_address, break_dest, continue_dest);
            result.ir_list.push(IrNode::Label(if_next_label));
            result.ir_list.append(&mut body_lin.ir_list);

            if else_body.node != AstNode::NullNode {
                result
                    .ir_list
                    .push(IrNode::Branch(CompareType::Always, expr.type_ref.clone(), 0, 0, if_end_label));
                result.ir_list.push(IrNode::Label(if_else_label));
                let mut else_lin = self.linearize(else_body, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut else_lin.ir_list);
            }
            result.ir_list.push(IrNode::Label(if_end_label));

            return result;
        }

        panic!("AstNode is not of type IfNode");
    }

    /// Lirgen::linearize_procedure_node
    ///
    /// Linearize a node of type ProcedureNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_procedure_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::ProcedureNode(primary, params) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            let mut list_params: Vec<u32> = vec![];
            if let AstNode::PrimaryNode(tk) = &primary.node {
                let id = Lirgen::get_identifier(tk);
                for p in params {
                    let mut linearized = self.linearize(p, get_address, break_dest, continue_dest);
                    result.ir_list.append(&mut linearized.ir_list);
                    list_params.push(linearized.result_register);
                }

                let result_register = self.get_register();
                result.ir_list.push(IrNode::Call(id, ast.type_ref.clone(), list_params, result_register));
                result.result_register = result_register;
                return result;
            } else {
                panic!("AstNode is not an identifier when calling function");
            }
        }

        panic!("AstNode is not of type ProcedureNode");
    }

    /// Lirgen::linearize_jump_node
    ///
    /// Linearize a node of type JumpNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_jump_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::JumpNode(token, expr) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            match token.tk {
                Tk::Keyword(Keyword::Return) => {
                    if expr.node != AstNode::NullNode {
                        let mut e_lin = self.linearize(expr, get_address, break_dest, continue_dest);
                        result.ir_list.append(&mut e_lin.ir_list);
                        result.ir_list.push(IrNode::Return(expr.type_ref.clone(), e_lin.result_register));
                    } else {
                        result.ir_list.push(IrNode::Return(expr.type_ref.clone(), 0));
                    }
                }
                Tk::Keyword(Keyword::Continue) => {
                    result
                        .ir_list
                        .push(IrNode::Branch(CompareType::Always, ast.type_ref.clone(), 0, 0, continue_dest.clone()));
                }
                Tk::Keyword(Keyword::Break) => {
                    result
                        .ir_list
                        .push(IrNode::Branch(CompareType::Always, ast.type_ref.clone(), 0, 0, break_dest.clone()));
                }
                _ => panic!("Invalid keyword {} in JumpNOde", token.tk),
            }
            return result;
        }

        panic!("AstNode is not of type ExprStatementNode");
    }

    /// Lirgen::linearize_expr_statement_node
    ///
    /// Linearize a node of type ExprStatementNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_expr_statement_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::ExprStatementNode(expr) = &ast.node {
            return self.linearize(expr, get_address, break_dest, continue_dest);
        }

        panic!("AstNode is not of type ExprStatementNode");
    }

    /// Lirgen::linearize_compound_node
    ///
    /// Linearize a node of type CompoundNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_compound_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::CompoundNode(list) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };

            for l in list {
                let mut l_lin = self.linearize(l, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut l_lin.ir_list);
            }

            return result;
        }

        panic!("AstNode is not of type CompoundNode");
    }

    /// Lirgen::linearize_declaration_list
    ///
    /// Linearize a node of type DeclarationList
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_declaration_list(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        // !!! So far the gloabl delcarations are skipped, as I am not sure how to handle them
        let mut result = LirgenResult { ..Default::default() };
        let mut functions_decl: Vec<IrNode> = vec![];

        if let AstNode::DeclarationList(list) = &ast.node {
            for elem in list {
                if let AstNode::FuncDeclNode(..) = &elem.node {
                    let mut lin = self.linearize(&elem, get_address, break_dest, continue_dest);
                    functions_decl.append(&mut lin.ir_list);
                }
                self.erase_registers();
            }

            result.ir_list.append(&mut functions_decl);
            return result;
        }

        panic!("AstNode is not of type DeclarationList");
    }

    /// Lirgen::linearize_var_decl_node
    ///
    /// Linearize a node of type VarDeclNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_var_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::VarDeclNode(tt, name, expression) = &ast.node {
            let mut result: LirgenResult = Default::default();
            let init_register;
            if expression.node != AstNode::NullNode {
                let mut expression_lin = self.linearize(expression, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut expression_lin.ir_list);
                init_register = expression_lin.result_register;
            } else {
                init_register = 0;
            }
            let result_register = self.get_register();
            let store_node = Alloc(tt.type_ref.clone(), result_register, init_register, false, 1, false);
            result.ir_list.push(store_node);
            result.result_register = result_register;
            self.add_pointer_variable(&Lirgen::get_identifier(name), result_register);
            return result;
        }

        panic!("AstNode is not of type VarDeclNode");
    }

    /// Lirgen::linearize_primary_node
    ///
    /// Linearize a node of type PrimaryNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_primary_node(&mut self, ast: &AstNodeWrapper, get_address: bool, _break_dest: u32, _continue_dest: u32) -> LirgenResult {
        if let AstNode::PrimaryNode(token) = &ast.node {
            match &token.tk {
                Tk::Identifier(id) => {
                    let mut result: LirgenResult = Default::default();
                    let load_register;
                    match self.get_pointer_variable(&id) {
                        Some(l) => {
                            load_register = l;
                            result.result_register = l;
                        }
                        None => {
                            load_register = self.get_register();
                            let mut tt = ast.type_ref.clone();
                            tt.pointer += 1;
                            let load_node = LoadA(tt, load_register, id.to_string());
                            result.ir_list.push(load_node);
                            result.result_register = load_register;
                        }
                    }
                    if get_address {
                        return result;
                    }
                    let result_register = self.get_register();
                    let load_node = LoadR(ast.type_ref.clone(), result_register, load_register);
                    result.ir_list.push(load_node);
                    result.result_register = result_register;
                    return result;
                }
                Tk::IntegerLiteral(num) => {
                    let result_register = self.get_register();
                    let store_constant_node = MovC(ast.type_ref.clone(), result_register, *num as u32);
                    return LirgenResult {
                        ir_list: vec![store_constant_node],
                        result_register,
                    };
                }
                Tk::Char(c) => {
                    let result_register = self.get_register();
                    let store_constant_node = MovC(ast.type_ref.clone(), result_register, *c as u32);
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

    /// Lirgen::linearize_binary_node
    ///
    /// Linearize a node of type BinaryNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_binary_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::BinaryNode(token, exp1, exp2) = &ast.node {
            let mut result: LirgenResult = Default::default();

            if let Tk::Operator(Operator::Assign) = token.tk {
                let mut exp1_lin = self.linearize(&exp1, true, break_dest, continue_dest);
                let mut exp2_lin = self.linearize(&exp2, get_address, break_dest, continue_dest);

                let new_op = IrNode::Store(ast.type_ref.clone(), exp1_lin.result_register, exp2_lin.result_register);

                result.ir_list.append(&mut exp1_lin.ir_list);
                result.ir_list.append(&mut exp2_lin.ir_list);
                result.ir_list.push(new_op);

                if get_address {
                    result.result_register = exp1_lin.result_register;
                } else {
                    result.result_register = exp2_lin.result_register;
                }

                return result;
            }

            let mut exp1_lin = self.linearize(&exp1, get_address, break_dest, continue_dest);
            let mut exp2_lin = self.linearize(&exp2, get_address, break_dest, continue_dest);
            let operator = Lirgen::get_operator(&token);

            let result_register = self.get_register();

            let new_op = IrNode::Binary(
                operator,
                ast.type_ref.clone(),
                result_register,
                exp1_lin.result_register,
                exp2_lin.result_register,
            );

            result.ir_list.append(&mut exp1_lin.ir_list);
            result.ir_list.append(&mut exp2_lin.ir_list);
            result.ir_list.push(new_op);
            result.result_register = result_register;

            return result;
        }

        panic!("AstNode is not of type BinaryNode");
    }

    /// Lirgen::linearize_cast_node
    ///
    /// Linearize a node of type CastNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_cast_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::CastNode(dest_type, exp) = &ast.node {
            let mut exp_lin = self.linearize(&exp, get_address, break_dest, continue_dest);

            if dest_type.type_ref.pointer > 0 {
                return exp_lin;
            }

            let result_register = self.get_register();
            let new_cast = IrNode::Cast(dest_type.type_ref.clone(), exp.type_ref.clone(), result_register, exp_lin.result_register);

            let mut ir_list: Vec<IrNode> = vec![];

            ir_list.append(&mut exp_lin.ir_list);
            ir_list.push(new_cast);

            return LirgenResult { ir_list, result_register };
        }

        panic!("AstNode is not of type CastNode");
    }

    /// Lirgen::linearize_func_decl_node
    ///
    /// Linearize a node of type FuncDeclNode
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    fn linearize_func_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::FuncDeclNode(rt, name, params, body) = &ast.node {
            let mut result = LirgenResult { ..Default::default() };
            let mut tt_list: Vec<TypeWrapper> = vec![];
            let mut ir_list: Vec<IrNode> = vec![];

            for i in 0..params.len() {
                let name_param;
                if let AstNode::ParameterNode(tk, ..) = &params[i].node {
                    name_param = Lirgen::get_identifier(&tk);
                } else {
                    panic!("Node not of type ParameterNode")
                }
                tt_list.push(params[i].type_ref.clone());
                let store_node = Alloc(
                    params[i].type_ref.clone(),
                    i as u32 + 1 as u32 + params.len() as u32,
                    i as u32 + 1,
                    false,
                    1,
                    false,
                );
                self.add_pointer_variable(&name_param, i as u32 + 1);
                ir_list.push(store_node);
            }

            self.current_register += params.len() as u32 * 2;

            let mut body_lin = self.linearize(body, get_address, break_dest, continue_dest);
            ir_list.append(&mut body_lin.ir_list);

            let func_node = IrNode::FunctionDeclaration(Lirgen::get_identifier(name), rt.type_ref.clone(), tt_list, ir_list.clone());
            result.ir_list.push(func_node);

            return result;
        }

        panic!("AstNode is not of type FuncDeclNode");
    }

    /// Lirgen::get_identifier
    ///
    /// Return the identifier contained in a token. Error if the token is not of type Identifier
    ///
    /// @in token[&Token]: token to use
    /// @return [String]: result of the extraction
    fn get_identifier(token: &Token) -> String {
        if let Tk::Identifier(s) = &token.tk {
            return s.clone();
        }
        panic!("Cannot extract identifier from non-identifier token: {:#?}", token);
    }

    /// Lirgen::get_operator
    ///
    /// Return the operator contained in a token. Error if the token is not of type operator
    ///
    /// @in token[&Token]: token to use
    /// @return [Operator]: result of the extraction
    fn get_operator(token: &Token) -> Operator {
        if let Tk::Operator(o) = &token.tk {
            return o.clone();
        }
        panic!("Cannot extract operator from non-identifier token: {:#?}", token);
    }
}

impl IrNode {
    pub fn to_string(&self) -> String {
        match &self {
            Program(list) => {
                let mut result = "".to_string();
                for l in list {
                    result += &l.to_string();
                }
                return result;
            }
            FunctionDeclaration(name, tt, arguments, nodes) => {
                let mut result = format!("\nfunction<{}> {} (", tt.to_string(), name);
                for i in 0..arguments.len() {
                    result += &format!("v{}<{}>", i + 1, arguments[i].to_string());
                    if i != arguments.len() - 1 {
                        result += &format!(", ");
                    }
                }
                result += &format!(") {{\n");

                for node in nodes {
                    result += &node.to_string();
                }

                return result + &format!("}}\n");
            }
            Return(tt, src) => {
                if *src != 0 {
                    return format!("\treturn<{}> v{}\n", tt.to_string(), src);
                }
                return format!("\treturn\n");
            }
            Alloc(tt, dest, src, is_global, size, from_reg) => {
                let mut result = format!("\tv{} = alloc<{}> ", dest, tt.to_string());
                if *src != 0 {
                    result += &format!("v{} ", src);
                }
                if *from_reg {
                    result += &format!("[v{}] ", size);
                } else {
                    result += &format!("[{}] ", size);
                }
                if *is_global {
                    result += &format!(" !global ");
                }

                return result + &format!("\n");
            }
            MovC(tt, dest, src) => {
                return format!("\tv{} = <{}> ${}\n", dest, tt.to_string(), src);
            }
            Cast(ttd, tts, dest, src) => {
                return format!("\tv{} = <{}><{}> v{}\n", dest, ttd.to_string(), tts.to_string(), src);
            }
            Store(tt, dest, src) => {
                return format!("\tstore<{}> v{}, v{}\n", tt.to_string(), dest, src);
            }
            LoadA(tt, dest, src) => {
                return format!("\tv{} = load<{}> @{}\n", dest, tt.to_string(), src);
            }
            LoadR(tt, dest, src) => {
                return format!("\tv{} = load<{}> v{}\n", dest, tt.to_string(), src);
            }
            Label(s) => {
                return format!("\n\t%L_{}:\n", s);
            }
            Call(name, tt, arguments, ret) => {
                let mut result = format!("\tv{} = call<{}> {}(", ret, tt.to_string(), name);
                for i in 0..arguments.len() {
                    result += &format!("v{}", arguments[i]);
                    if i != arguments.len() - 1 {
                        result += &format!(", ");
                    }
                }

                return result + &format!(")\n");
            }
            Branch(ct, tt, src1, src2, name) => {
                let mut result = format!("\tj{}", ct.to_string());

                match *ct {
                    CompareType::Always => {}
                    CompareType::S | CompareType::NS => result += &format!("<{}> v{}", tt.to_string(), src1),
                    _ => result += &format!("<{}> v{}, v{}", tt.to_string(), src1, src2),
                }

                return result + &format!(" %L_{}\n", name);
            }
            Unary(tt, tk, dest, src) => {
                let mut result = format!("\tv{} = ", dest);
                match tk {
                    Operator::Minus => result += "neg",
                    Operator::Plus => result += "plus",
                    Operator::Complement => result += "comp",
                    Operator::Not => result += "not",
                    _ => panic!("Invalid binary operator {:#?}", tk),
                }
                result += &format!("<{}> v{}\n", tt.to_string(), src);
                return result;
            }
            Binary(tk, tt, dest, src1, src2) => {
                let mut result = format!("\tv{} = ", dest);

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

                result += &format!(" <{}> v{}, v{}\n", tt.to_string(), src1, src2);

                return result;
            }
        }
    }
}

impl CompareType {
    fn from_token(t: &Token) -> CompareType {
        match t.tk {
            Tk::Operator(Operator::GECompare) => CompareType::GE,
            Tk::Operator(Operator::GTCompare) => CompareType::GT,
            Tk::Operator(Operator::LECompare) => CompareType::LE,
            Tk::Operator(Operator::LTCompare) => CompareType::LT,
            Tk::Operator(Operator::EqualCompare) => CompareType::EQ,
            Tk::Operator(Operator::DiffCompare) => CompareType::NE,
            _ => panic!("Cannot covert token {:?} into CompareType", t.tk),
        }
    }

    fn to_string(&self) -> String {
        match *self {
            CompareType::Always => return "".to_string(),
            CompareType::GT => return "gt".to_string(),
            CompareType::GE => return "ge".to_string(),
            CompareType::LT => return "lt".to_string(),
            CompareType::LE => return "le".to_string(),
            CompareType::S => return "s".to_string(),
            CompareType::NS => return "ns".to_string(),
            CompareType::EQ => return "eq".to_string(),
            CompareType::NE => return "ne".to_string(),
        }
    }

    fn opposite(&self) -> CompareType {
        match *self {
            CompareType::Always => CompareType::Always,
            CompareType::GT => CompareType::LE,
            CompareType::GE => CompareType::LT,
            CompareType::LT => CompareType::GE,
            CompareType::LE => CompareType::GT,
            CompareType::S => CompareType::NS,
            CompareType::NS => CompareType::S,
            CompareType::EQ => CompareType::NE,
            CompareType::NE => CompareType::EQ,
        }
    }
}
