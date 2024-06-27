use crate::ast::ast_impl::TypeWrapper;
use crate::lexer::lexer_impl::{Operator, Tk, Token};

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
use IrNode::*;

impl IrNode {
    /// IrNode::to_string
    ///
    /// Get a string out of an IrNode
    ///
    /// @return [String]: result of the conversion
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
    /// CompareType::from_token
    ///
    /// @in t [&Token]: token to use
    /// @result [CompareType]: Equivalent compare type
    pub fn from_token(t: &Token) -> CompareType {
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
