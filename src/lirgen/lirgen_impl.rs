use crate::ast::ast_impl::{AstNode, AstNodeWrapper, TypeWrapper};
use crate::lexer::lexer_impl::{Keyword, Operator, Tk, Token};
use crate::lirgen::irnode::{CompareType, IrNode};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LirgenResult {
    pub ir_list: Vec<IrNode>,
    pub result_register: u32,
}

pub struct Lirgen {
    current_register: u32,
    current_label: u32,
    variable_pointers: Vec<(String, u32)>,
    variable_values: Vec<(String, u32)>,
    constant_values: Vec<(u32, u32)>,
    computed_binary: Vec<(Operator, u32, u32, u32)>,
    is_global: bool,
    to_invalidate: bool,
    to_invalidate_variable: Vec<String>,
    to_invalidate_constant: Vec<u32>,
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
            variable_values: vec![],
            constant_values: vec![],
            computed_binary: vec![],
            to_invalidate_variable: vec![],
            to_invalidate_constant: vec![],
            is_global: false,
            to_invalidate: false,
        };
    }

    fn invalidate(&mut self) {
        for elem in &self.to_invalidate_constant {
            for i in 0..self.constant_values.len() {
                if elem == &self.constant_values[i].0 {
                    self.constant_values.remove(i);
                    break;
                }
            }
        }

        for elem in &self.to_invalidate_variable {
            for i in 0..self.variable_values.len() {
                if elem == &self.variable_values[i].0 {
                    self.variable_values.remove(i);
                    break;
                }
            }
        }
    }

    /// Lirgen::get_pointer_variable
    ///
    /// Add a stored pointer to variable
    ///
    /// @in s[&String]: name of the pointed variable
    /// @return [Option<u32>]: None if the pointer is not stored, otherwise the register having it
    fn get_pointer_variable(&self, s: &String) -> Option<u32> {
        for elem in &self.variable_pointers {
            if *elem.0 == *s {
                return Some(elem.1);
            }
        }
        return None;
    }

    /// Lirgen::get_variables
    ///
    /// Add a stored variable
    ///
    /// @in s[&String]: name of the variable
    /// @return [Option<u32>]: None if the variable is not stored, otherwise the register having it
    fn get_variables(&self, s: &String) -> Option<u32> {
        for elem in &self.variable_values {
            if *elem.0 == *s {
                return Some(elem.1);
            }
        }
        return None;
    }

    /// @in s[(Operator, u32, u32, u32)]: computed binary in the format (op, dest, src1, src2)
    ///
    /// Lirgen::get_computed_binary
    ///
    /// Search an already computed binary
    ///
    /// @in s1[u32]: first source
    /// @in s2[u32]: second source
    /// @in op[u32]: operator
    /// @return [Option<u32>]: None if the variable is not stored, otherwise the register having it
    fn get_computed_binary(&self, s1: u32, s2: u32, op: &Operator) -> Option<u32> {
        for elem in &self.computed_binary {
            if elem.0 == *op && elem.2 == s1 && elem.3 == s2 {
                return Some(elem.1);
            }
            if *op == Operator::Plus || *op == Operator::Asterisk || *op == Operator::AndOp || *op == Operator::OrOp || *op == Operator::XorOp {
                if elem.0 == *op && elem.3 == s1 && elem.2 == s2 {
                    return Some(elem.1);
                }
            }
        }
        return None;
    }

    /// Lirgen::get_constant
    ///
    /// Add a stored constant
    ///
    /// @in c[u32]: value of the constant
    /// @return [Option<u32>]: None if the constant is not stored, otherwise the register having it
    fn get_constant(&self, c: u32) -> Option<u32> {
        for elem in &self.constant_values {
            if elem.0 == c {
                return Some(elem.1);
            }
        }
        return None;
    }

    /// Lirgen::add_constant
    ///
    /// Add a stored constant
    ///
    /// @in r[u32]: register in which the constant is stored
    /// @in v[u32]: value of the constant
    fn add_constant(&mut self, r: u32, v: u32) {
        if self.to_invalidate {
            self.to_invalidate_constant.push(v);
        }
        self.constant_values.push((v, r));
    }

    /// Lirgen::add_variable
    ///
    /// Add a stored variable
    ///
    /// @in s[&String]: name of the variable
    /// @in r[u32]: register in which the variable is stored
    fn add_variable(&mut self, s: &String, r: u32) {
        if self.to_invalidate {
            self.to_invalidate_variable.push(s.clone());
        }
        self.variable_values.push((s.clone(), r));
    }

    /// Lirgen::clear_variable_values
    ///
    /// Remove all the stored variables
    fn clear_variable_values(&mut self) {
        self.variable_values.clear();
    }

    /// Lirgen::add_pointer_variable
    ///
    /// Add a stored pointer
    ///
    /// @in s[&String]: name of the variable pointed by the pointer
    /// @in r[u32]: register in which the pointer is stored
    fn add_pointer_variable(&mut self, s: &String, r: u32) {
        self.variable_pointers.push((s.clone(), r));
    }

    /// Lirgen::add_computed_binary
    ///
    /// Add a computed binary
    ///
    /// @in s[(Operator, u32, u32, u32)]: computed binary in the format (op, dest, src1, src2)
    fn add_computed_binary(&mut self, s: (Operator, u32, u32, u32)) {
        self.computed_binary.push(s.clone());
    }

    /// Lirgen::erase_registers
    ///
    /// Get rid of all the information stored in the IR generator
    fn erase_registers(&mut self) {
        self.current_register = 0;
        self.current_label = 0;
        self.variable_pointers.clear();
        self.variable_values.clear();
        self.constant_values.clear();
        self.computed_binary.clear();
    }

    /// Lirgen::get_register
    ///
    /// Get the next register to use
    ///
    /// @return [u32]: register
    fn get_register(&mut self) -> u32 {
        self.current_register += 1;
        return self.current_register;
    }

    /// Lirgen::get_label
    ///
    /// Get the next label to use
    ///
    /// @return [u32]: label
    fn get_label(&mut self) -> u32 {
        self.current_label += 1;
        return self.current_label;
    }

    /// Lirgen::linearize_ast
    ///
    /// Get the linearized version of the input ast
    ///
    /// @in ast[&AstNodeWrapper]: ast to linearize
    /// @return [IrNode]: linearized version of type IrNode::Program
    pub fn linearize_ast(&mut self, ast: &AstNodeWrapper) -> IrNode {
        IrNode::Program(self.linearize(ast, false, 0, 0).ir_list)
    }

    /// Lirgen::linearize
    ///
    /// Linearize an ast node
    ///
    /// @in ast[&AstNodeWrapper]: node to linearize
    /// @in get_address[bool]: in case of an expression, whether we have to extract the address of
    /// the operand (in case of an lvalue) or its value
    /// @in break_dest[u32]: in case of a loop, label to jump for break instructions
    /// @in continue_dest[u32]: in case of a loop, label to jump for continue instructions
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
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

            let constant_register;
            match self.get_constant(size) {
                Some(l) => {
                    constant_register = l;
                }
                None => {
                    let result_register = self.get_register();
                    let store_constant_node = MovC(tt.clone(), result_register, size);
                    result.ir_list.push(store_constant_node);
                    constant_register = result_register;
                    self.add_constant(result_register, size);
                }
            }

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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
    fn linearize_array_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::ArrayDeclNode(tt, name, expression) = &ast.node {
            let mut result: LirgenResult = Default::default();
            let mut expression_lin = self.linearize(&expression, get_address, break_dest, continue_dest);
            let init_register = expression_lin.result_register;
            let result_register = self.get_register();
            let store_node = Alloc(tt.type_ref.clone(), result_register, 0, self.is_global, init_register, true);

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
    /// @return [LirgenResult]: result of the conversion
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

            let old_to_invalidate = self.to_invalidate;
            let old_to_invalidate_c = self.to_invalidate_constant.clone();
            self.to_invalidate = true;
            self.to_invalidate_variable = vec![];
            self.variable_values.clear();

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

            self.invalidate();
            self.to_invalidate = old_to_invalidate;
            self.to_invalidate_constant = old_to_invalidate_c;
            return result;
        }
        panic!("AstNode is not of type ForNode");
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
    /// @return [LirgenResult]: result of the conversion
    fn linearize_while_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::WhileNode(expr, body) = &ast.node {
            let old_to_invalidate = self.to_invalidate;
            let old_to_invalidate_c = self.to_invalidate_constant.clone();
            self.to_invalidate = true;
            self.to_invalidate_constant = vec![];
            self.variable_values.clear();

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

            self.invalidate();
            self.to_invalidate = old_to_invalidate;
            self.to_invalidate_constant = old_to_invalidate_c;

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
    /// @return [LirgenResult]: result of the conversion
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

            let old_to_invalidate = self.to_invalidate;
            let old_to_invalidate_c = self.to_invalidate_constant.clone();
            let old_to_invalidate_v = self.to_invalidate_variable.clone();
            self.to_invalidate = true;
            self.to_invalidate_variable = vec![];
            self.to_invalidate_constant = vec![];

            let mut body_lin = self.linearize(body, get_address, break_dest, continue_dest);
            result.ir_list.push(IrNode::Label(if_next_label));
            result.ir_list.append(&mut body_lin.ir_list);

            self.invalidate();
            self.to_invalidate = old_to_invalidate;
            self.to_invalidate_constant = old_to_invalidate_c;
            self.to_invalidate_variable = old_to_invalidate_v;

            if else_body.node != AstNode::NullNode {
                let old_to_invalidate = self.to_invalidate;
                let old_to_invalidate_c = self.to_invalidate_constant.clone();
                let old_to_invalidate_v = self.to_invalidate_variable.clone();
                self.to_invalidate = true;
                self.to_invalidate_variable = vec![];
                self.to_invalidate_constant = vec![];

                result
                    .ir_list
                    .push(IrNode::Branch(CompareType::Always, expr.type_ref.clone(), 0, 0, if_end_label));
                result.ir_list.push(IrNode::Label(if_else_label));
                let mut else_lin = self.linearize(else_body, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut else_lin.ir_list);

                self.invalidate();
                self.to_invalidate = old_to_invalidate;
                self.to_invalidate_constant = old_to_invalidate_c;
                self.to_invalidate_variable = old_to_invalidate_v;
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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
    fn linearize_declaration_list(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        let mut result = LirgenResult { ..Default::default() };
        let mut functions_decl: Vec<IrNode> = vec![];
        let mut var_decl: Vec<IrNode> = vec![];

        if let AstNode::DeclarationList(list) = &ast.node {
            for elem in list {
                if let AstNode::FuncDeclNode(..) = &elem.node {
                    let mut lin = self.linearize(&elem, get_address, break_dest, continue_dest);
                    functions_decl.append(&mut lin.ir_list);
                    var_decl.append(&mut lin.ir_list);
                    self.erase_registers();
                }
            }

            for elem in list {
                if let AstNode::FuncDeclNode(..) = &elem.node {
                } else {
                    self.is_global = true;
                    let mut lin = self.linearize(&elem, get_address, break_dest, continue_dest);
                    self.is_global = true;
                    var_decl.append(&mut lin.ir_list);
                }
            }

            var_decl.push(IrNode::Call("main".to_string(), TypeWrapper { ..Default::default() }, vec![], 0));
            var_decl.push(Label(0));
            var_decl.push(Branch(CompareType::Always, TypeWrapper { ..Default::default() }, 0, 0, 0));
            let init_node = IrNode::FunctionDeclaration("init".to_string(), TypeWrapper { ..Default::default() }, vec![], var_decl);

            result.ir_list.push(init_node);
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
    /// @return [LirgenResult]: result of the conversion
    fn linearize_var_decl_node(&mut self, ast: &AstNodeWrapper, get_address: bool, break_dest: u32, continue_dest: u32) -> LirgenResult {
        if let AstNode::VarDeclNode(tt, name, expression) = &ast.node {
            let mut result: LirgenResult = Default::default();
            let init_register;
            if expression.node != AstNode::NullNode {
                let mut expression_lin = self.linearize(expression, get_address, break_dest, continue_dest);
                result.ir_list.append(&mut expression_lin.ir_list);
                init_register = expression_lin.result_register;
                self.add_variable(&Lirgen::get_identifier(name), init_register);
            } else {
                init_register = 0;
            }
            let result_register = self.get_register();
            let store_node = Alloc(tt.type_ref.clone(), result_register, init_register, self.is_global, 1, false);
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
    /// @return [LirgenResult]: result of the conversion
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
                    if get_address || ast.type_ref.pointer > 0 {
                        return result;
                    }
                    match self.get_variables(&id) {
                        Some(l) => {
                            result.result_register = l;
                        }
                        None => {
                            let result_register = self.get_register();
                            let load_node = LoadR(ast.type_ref.clone(), result_register, load_register);
                            result.ir_list.push(load_node);
                            result.result_register = result_register;
                            self.add_variable(id, result_register);
                        }
                    }
                    return result;
                }
                Tk::IntegerLiteral(num) => {
                    let mut result: LirgenResult = Default::default();
                    match self.get_constant(*num as u32) {
                        Some(l) => {
                            result.result_register = l;
                        }
                        None => {
                            let result_register = self.get_register();
                            let store_constant_node = MovC(ast.type_ref.clone(), result_register, *num as u32);
                            result.ir_list.push(store_constant_node);
                            result.result_register = result_register;
                            self.add_constant(result_register, *num as u32);
                        }
                    }
                    return result;
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
    /// @return [LirgenResult]: result of the conversion
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

                match &exp1.node {
                    AstNode::PrimaryNode(tk) => {
                        let id = Lirgen::get_identifier(tk);
                        let mut found = false;
                        for i in 0..self.variable_values.len() {
                            if self.variable_values[i].0 == id {
                                self.variable_values[i].1 = exp2_lin.result_register;
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            self.add_variable(&id.clone(), exp2_lin.result_register);
                        }

                        self.to_invalidate_variable.push(id.clone());
                    }
                    _ => self.clear_variable_values(),
                }

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

            match self.get_computed_binary(exp1_lin.result_register, exp2_lin.result_register, &operator) {
                Some(dest) => {
                    result.result_register = dest;
                    return result;
                }
                None => {}
            }

            let result_register = self.get_register();
            self.add_computed_binary((operator.clone(), result_register, exp1_lin.result_register, exp2_lin.result_register));

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
    /// @return [LirgenResult]: result of the conversion
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
    /// @return [LirgenResult]: result of the conversion
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
                self.add_pointer_variable(&name_param, i as u32 + 1 + params.len() as u32);
                self.add_variable(&name_param, i as u32 + 1);
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
