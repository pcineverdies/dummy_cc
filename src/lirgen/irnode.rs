use crate::ast::type_wrapper::TypeWrapper;
use crate::lexer::token::{Operator, Tk, Token};

/// enum CompareType
///
/// Enum associated to the different conditions for a branch
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompareType {
    Always, // Branch is always taken
    GT,     // Branch is taken iff src1 > src2
    GE,     // Branch is taken iff src1 >= src2
    LT,     // Branch is taken iff src1 < src2
    LE,     // Branch is taken iff src1 <= src2
    EQ,     // Branch is taken iff src1 == src2
    NE,     // Branch is taken iff src1 != src2
    S,      // Branch is taken iff src1 != 0
    NS,     // Branch is taken iff src1 == 0
}

/// enum IrNode
///
/// Enum associated to the different nodes avaialble in the Intermediate Representation adopted in
/// the compiler. The intermediate representation is a linearized form of the ast so that
/// optimizations are simple to implment together with an efficinet back-end system.
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

use IrNode::*;

impl IrNode {
    /// IrNode::get_dest
    ///
    /// Given an IrNode, get its destination register, 0 if the node has no destination register.
    /// @return [u32]: destination register
    pub fn get_dest(&self) -> u32 {
        match &self {
            Alloc(_, dest, ..) => return *dest,
            MovC(_, dest, ..) => return *dest,
            Cast(_, _, dest, ..) => return *dest,
            Store(_, dest, ..) => return *dest,
            LoadA(_, dest, ..) => return *dest,
            LoadR(_, dest, ..) => return *dest,
            Call(_, _, _, ret) => return *ret,
            Unary(_, _, dest, ..) => return *dest,
            Binary(_, _, dest, ..) => return *dest,
            _ => return 0,
        }
    }

    /// IrNode::get_src
    ///
    /// Given an IrNode, get its source registers (might be more than one); an empty vector is
    /// returned if the node has no sources
    /// @return [Vec<u32>]: source registers
    pub fn get_src(&self) -> Vec<u32> {
        match &self {
            Return(_, src) => return vec![*src],
            Alloc(_, _, src, _, _, _) => return vec![*src],
            Cast(_, _, _, src) => return vec![*src],
            Store(_, _, src) => return vec![*src],
            LoadR(_, _, src) => return vec![*src],
            Call(_, _, arguments, _) => return arguments.clone(),
            Branch(_, _, src1, src2, _) => return vec![*src1, *src2],
            Unary(_, _, _, src) => return vec![*src],
            Binary(_, _, _, src1, src2) => return vec![*src1, *src2],
            _ => return vec![],
        }
    }

    /// IrNode::to_string
    ///
    /// Get a string out of an IrNode
    ///
    /// @return [String]: result of the conversion
    pub fn to_string(&self) -> String {
        match &self {
            // In case of a program, print all the functions one after the other
            Program(list) => {
                let mut result = "".to_string();
                for l in list {
                    result += &l.to_string();
                }
                return result;
            }
            // For a function, print its declaration (name, return type and arguments) together
            // wiht all its statements
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
                // If the source register of a return statement is zero, then the statement is
                // associated to a void function
                if *src != 0 {
                    return format!("\treturn<{}> v{}\n", tt.to_string(), src);
                }
                return format!("\treturn\n");
            }
            Alloc(tt, dest, src, is_global, size, from_reg) => {
                let mut result = format!("\tv{} = alloc<{}> ", dest, tt.to_string());
                // No initizialization register
                if *src != 0 {
                    result += &format!("v{} ", src);
                }
                // Size depends on the value of a register
                if *from_reg {
                    result += &format!("[v{}] ", size);
                }
                // Global declaration
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
                    // No other arguments
                    CompareType::Always => {}
                    // One source register
                    CompareType::S | CompareType::NS => result += &format!("<{}> v{}", tt.to_string(), src1),
                    // Two source registers
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
    /// CompareType::from_token
    ///
    /// @in t [&Token]: token to use to extract the comparison type
    /// @result [CompareType]: Equivalent compare type
    pub fn from_token(t: &Token) -> Option<CompareType> {
        match t.tk {
            Tk::Operator(Operator::GECompare) => Some(CompareType::GE),
            Tk::Operator(Operator::GTCompare) => Some(CompareType::GT),
            Tk::Operator(Operator::LECompare) => Some(CompareType::LE),
            Tk::Operator(Operator::LTCompare) => Some(CompareType::LT),
            Tk::Operator(Operator::EqualCompare) => Some(CompareType::EQ),
            Tk::Operator(Operator::DiffCompare) => Some(CompareType::NE),
            _ => None,
        }
    }

    /// CompareType::to_string
    ///
    /// @result [String]: Equivalent string
    pub fn to_string(&self) -> String {
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

    /// CompareType::opposite
    ///
    /// @result [CompareType]: Opposite compare type with respect to the given one
    pub fn opposite(&self) -> CompareType {
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
