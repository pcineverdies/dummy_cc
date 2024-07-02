use std::collections::HashMap;

use crate::ast::type_wrapper::TypeWrapper;
use crate::backend::riscv_isa::{RiscvInstruction, RiscvInstructionType, A0, FP, GP, RA, SP, X0};
use crate::lexer::token::Operator;
use crate::lirgen::irnode::{CompareType, IrNode};

use IrNode::*;
use RiscvInstructionType::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StackOffset {
    size: u32,
    reg: u32,
    offset: i32,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Codegen {
    current_new_register: i32,
    constants_map: HashMap<u32, u32>,
    global_stack_position: Vec<StackOffset>,
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            current_new_register: 2_i32.pow(15),
            constants_map: HashMap::new(),
            global_stack_position: vec![],
        }
    }

    fn get_new_register(&mut self) -> i32 {
        let result = self.current_new_register;
        self.current_new_register += 1;
        return result;
    }

    fn get_pre_function(&self, name: &String, ssa: u32, args: &Vec<TypeWrapper>) -> Vec<RiscvInstruction> {
        let mut pre_function: Vec<RiscvInstruction> = vec![];
        // Add label function
        pre_function.push(RiscvInstruction {
            tt: LABELFUNCTION,
            name: name.to_string(),
            ..Default::default()
        });
        // Create the space for the actication record
        pre_function.push(RiscvInstruction {
            tt: ADDI,
            dest: SP,
            src1: SP,
            immediate: -(ssa as i32),
            ..Default::default()
        });
        // Store RA in the activatino record
        pre_function.push(RiscvInstruction {
            tt: SW,
            src1: SP,
            src2: RA,
            immediate: 4 as i32,
            ..Default::default()
        });
        // Store FP in the activation record
        pre_function.push(RiscvInstruction {
            tt: SW,
            src1: SP,
            src2: FP,
            immediate: 8 as i32,
            ..Default::default()
        });
        // Use FP as frame pointer (pointer to the start of the activation record)
        pre_function.push(RiscvInstruction {
            tt: ADDI,
            dest: FP,
            src1: SP,
            immediate: (ssa as i32),
            ..Default::default()
        });

        if args.len() >= 8 {
            for i in 8..args.len() {
                pre_function.push(RiscvInstruction {
                    tt: LW,
                    dest: i as i32,
                    src1: SP,
                    immediate: (i as i32 - 7) * 4,
                    ..Default::default()
                });
            }
        }

        if name == "init" {
            pre_function.insert(
                1,
                RiscvInstruction {
                    tt: ADDI,
                    dest: SP,
                    src1: X0,
                    immediate: 0x00010000,
                    ..Default::default()
                },
            );
            pre_function.push(RiscvInstruction {
                tt: ADDI,
                dest: GP,
                src1: FP,
                immediate: 0,
                ..Default::default()
            });
        }

        return pre_function;
    }

    fn get_post_function(&self, ssa: u32) -> Vec<RiscvInstruction> {
        let mut post_function: Vec<RiscvInstruction> = vec![];
        // Restore RA
        post_function.push(RiscvInstruction {
            tt: LW,
            dest: RA,
            src1: SP,
            immediate: 4 as i32,
            ..Default::default()
        });
        // Restore FP
        post_function.push(RiscvInstruction {
            tt: LW,
            dest: FP,
            src1: SP,
            immediate: 8 as i32,
            ..Default::default()
        });
        // Get the previous value of SP
        post_function.push(RiscvInstruction {
            tt: ADDI,
            dest: SP,
            src1: SP,
            immediate: (ssa as i32),
            ..Default::default()
        });
        // Return
        post_function.push(RiscvInstruction {
            tt: JAL,
            dest: X0,
            src1: RA,
            ..Default::default()
        });
        return post_function;
    }

    fn convert_node(&mut self, node: &IrNode, stack_position: &Vec<StackOffset>, i_function: u32) -> (Vec<RiscvInstruction>, Vec<RiscvInstruction>) {
        let mut in_function: Vec<RiscvInstruction> = vec![];
        let mut post_function: Vec<RiscvInstruction> = vec![];
        match node {
            Return(_, src) => {
                if *src != 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: A0,
                        src1: *src as i32,
                        ..Default::default()
                    });
                }
            }
            Alloc(tt, dest, src, _, size, from_reg, ..) => {
                if *from_reg {
                    let new_register = self.get_new_register();
                    // Make size multiple of 16
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: new_register,
                        src1: *size as i32,
                        immediate: 15 as i32,
                        ..Default::default()
                    });
                    in_function.push(RiscvInstruction {
                        tt: ANDI,
                        dest: new_register,
                        src1: *size as i32,
                        immediate: (0xfffffff0 as u32) as i32,
                        ..Default::default()
                    });
                    in_function.push(RiscvInstruction {
                        tt: SUB,
                        dest: SP,
                        src1: SP,
                        src2: new_register,
                        ..Default::default()
                    });
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: *dest as i32,
                        src1: SP,
                        immediate: 0,
                        ..Default::default()
                    });
                    post_function.insert(
                        0,
                        RiscvInstruction {
                            tt: ADD,
                            dest: SP,
                            src1: SP,
                            src2: new_register,
                            ..Default::default()
                        },
                    );
                }
                if *src == 0 {
                    return (in_function, post_function);
                }
                let mut store_instruction = RiscvInstruction { ..Default::default() };
                store_instruction.tt = match tt.get_size() {
                    4 => SW,
                    2 => SH,
                    _ => SB,
                };
                store_instruction.src2 = *src as i32;
                let mut found_register = false;
                for elem in stack_position {
                    if elem.reg == *dest {
                        store_instruction.src1 = FP;
                        store_instruction.immediate = elem.offset as i32;
                        found_register = true
                    }
                }
                if !found_register {
                    store_instruction.src1 = *dest as i32;
                }
                in_function.push(store_instruction);
            }
            MovC(_, dest, src) => {
                if *src > (1 << 12) {
                    in_function.push(RiscvInstruction {
                        tt: LUI,
                        dest: *dest as i32,
                        immediate: (src >> 12) as i32,
                        ..Default::default()
                    });
                } else {
                    self.constants_map.insert(*dest, *src);
                }
                in_function.push(RiscvInstruction {
                    tt: ADDI,
                    dest: *dest as i32,
                    src1: X0,
                    immediate: (src % (1 << 12)) as i32,
                    ..Default::default()
                });
            }
            Cast(ttd, tts, dest, src) => {
                if ttd.get_size() != 4 {
                    let and_mask = if ttd.get_size() == 1 { 0xff } else { 0xffff };
                    in_function.push(RiscvInstruction {
                        tt: ANDI,
                        dest: *dest as i32,
                        src1: *src as i32,
                        immediate: and_mask as i32,
                        ..Default::default()
                    });
                }
                let min_size = tts.get_size().min(ttd.get_size());
                if ttd.is_signed() {
                    let shift_size = 32 - min_size * 8;
                    if min_size != 4 {
                        in_function.push(RiscvInstruction {
                            tt: SLLI,
                            dest: *dest as i32,
                            src1: *src as i32,
                            immediate: shift_size as i32,
                            ..Default::default()
                        });
                        in_function.push(RiscvInstruction {
                            tt: SRLI,
                            dest: *dest as i32,
                            src1: *dest as i32,
                            immediate: shift_size as i32,
                            is_unsigned: false,
                            ..Default::default()
                        });
                    }
                }
            }
            Store(tt, dest, src) => {
                let mut store_instruction = RiscvInstruction { ..Default::default() };
                store_instruction.tt = match tt.get_size() {
                    4 => SW,
                    2 => SH,
                    _ => SB,
                };
                store_instruction.src2 = *src as i32;
                let mut found_register = false;
                for elem in stack_position {
                    if elem.reg == *dest {
                        store_instruction.src1 = FP;
                        store_instruction.immediate = elem.offset;
                        found_register = true
                    }
                }
                if !found_register {
                    store_instruction.src1 = *dest as i32;
                }
                in_function.push(store_instruction);
            }
            LoadA(_, dest, src) => {
                let mut load_instruction = RiscvInstruction { ..Default::default() };
                load_instruction.tt = ADDI;
                load_instruction.dest = *dest as i32;
                load_instruction.src1 = GP;
                for elem in &self.global_stack_position {
                    if elem.name == *src {
                        load_instruction.immediate = elem.offset as i32;
                    }
                }
                in_function.push(load_instruction);
            }
            LoadR(tt, dest, src) => {
                let mut load_instruction = RiscvInstruction { ..Default::default() };
                load_instruction.tt = match tt.get_size() {
                    4 => LW,
                    2 => LH,
                    _ => LB,
                };
                load_instruction.dest = *dest as i32;
                let mut found_register = false;
                for elem in stack_position {
                    if elem.reg == *src {
                        load_instruction.src1 = FP;
                        load_instruction.immediate = elem.offset as i32;
                        found_register = true
                    }
                }
                if !found_register {
                    load_instruction.src1 = *src as i32;
                }
                in_function.push(load_instruction);
            }
            Label(s) => in_function.push(RiscvInstruction {
                tt: LABEL,
                label: *s,
                label_function: i_function as u32,
                ..Default::default()
            }),
            Call(name, _, arguments, ret) => {
                let mut extra_space: i32 = 0;
                let mut extra_arguments: i32 = 0;
                for i in 0..arguments.len() {
                    if i < 8 {
                        in_function.push(RiscvInstruction {
                            tt: ADDI,
                            dest: A0 - i as i32,
                            src1: arguments[i] as i32,
                            immediate: 0,
                            ..Default::default()
                        });
                    } else {
                        extra_arguments = arguments.len() as i32 - 8;
                        extra_space = ((extra_arguments * 4) + 15) & 0xffffff0 as i32;
                        in_function.push(RiscvInstruction {
                            tt: SW,
                            src1: SP,
                            src2: arguments[i] as i32,
                            immediate: -(extra_space as i32) + (i as i32 - 7) * 4 as i32,
                            ..Default::default()
                        });
                        extra_arguments += 1;
                    }
                }
                if extra_arguments > 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: SP,
                        src1: SP,
                        immediate: -extra_space,
                        ..Default::default()
                    });
                }
                in_function.push(RiscvInstruction {
                    tt: JAL,
                    dest: RA,
                    name: name.to_string(),
                    ..Default::default()
                });
                if *ret != 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: *ret as i32,
                        src1: A0,
                        immediate: 0,
                        ..Default::default()
                    });
                }
                if extra_arguments > 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: SP,
                        src1: SP,
                        immediate: extra_space,
                        ..Default::default()
                    });
                }
            }
            Branch(ct, tt, src1, src2, name) => {
                let mut branch_instruction = RiscvInstruction { ..Default::default() };
                branch_instruction.label = *name;
                branch_instruction.label_function = i_function as u32;
                branch_instruction.src1 = *src1 as i32;
                branch_instruction.src2 = *src2 as i32;
                match ct {
                    CompareType::Always => {
                        branch_instruction.tt = J;
                    }
                    CompareType::GT => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BLT;
                        branch_instruction.src2 = *src1 as i32;
                        branch_instruction.src1 = *src2 as i32;
                    }
                    CompareType::GE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BGE;
                    }
                    CompareType::LT => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BLT;
                    }
                    CompareType::LE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BGE;
                        branch_instruction.src2 = *src1 as i32;
                        branch_instruction.src1 = *src2 as i32;
                    }
                    CompareType::EQ => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BEQ;
                    }
                    CompareType::NE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BNE;
                    }
                    CompareType::S => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BNE;
                        branch_instruction.src2 = X0;
                    }
                    CompareType::NS => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BEQ;
                        branch_instruction.src2 = X0;
                    }
                }
                in_function.push(branch_instruction);
            }
            Unary(_, tk, dest, src) => {
                match tk {
                    // dest = 0 - source
                    Operator::Minus => in_function.push(RiscvInstruction {
                        tt: SUB,
                        dest: *dest as i32,
                        src1: X0,
                        src2: *src as i32,
                        ..Default::default()
                    }),
                    // dest = source ^ 0xffffffff
                    Operator::Complement => in_function.push(RiscvInstruction {
                        tt: XORI,
                        dest: *dest as i32,
                        src1: *src as i32,
                        immediate: (0xffffffff as u32) as i32,
                        ..Default::default()
                    }),
                    // dest = if source < 1 {1} else {0} => set if zero, clear if not zero
                    Operator::Not => in_function.push(RiscvInstruction {
                        tt: SLTI,
                        dest: *dest as i32,
                        src1: *src as i32,
                        immediate: 1,
                        is_unsigned: true,
                        ..Default::default()
                    }),
                    _ => panic!("Invalid binary operator {:#?}", tk),
                }
            }
            Binary(tk, tt, dest, src1, src2) => {
                let mut binary_instruction = RiscvInstruction { ..Default::default() };
                binary_instruction.dest = *dest as i32;
                binary_instruction.src1 = *src1 as i32;
                binary_instruction.src2 = *src2 as i32;
                binary_instruction.is_unsigned = !tt.is_signed();
                let mut to_add = true;
                match tk {
                    // dest = src1 - src2
                    // dest = if dest == 0 {1} else {0}
                    Operator::EqualCompare => {
                        in_function.push(RiscvInstruction {
                            tt: SUB,
                            dest: *dest as i32,
                            src1: *src1 as i32,
                            src2: *src2 as i32,
                            ..Default::default()
                        });
                        in_function.push(RiscvInstruction {
                            tt: SLTI,
                            dest: *dest as i32,
                            src1: *dest as i32,
                            immediate: 1,
                            is_unsigned: true,
                            ..Default::default()
                        });
                        to_add = false;
                    }
                    // dest = src1 - src2
                    // dest = if dest == 0 {0} else {1}
                    Operator::DiffCompare => {
                        in_function.push(RiscvInstruction {
                            tt: SUB,
                            dest: *dest as i32,
                            src1: *src1 as i32,
                            src2: *src2 as i32,
                            ..Default::default()
                        });
                        in_function.push(RiscvInstruction {
                            tt: SLT,
                            dest: *dest as i32,
                            src1: X0,
                            src2: *dest as i32,
                            is_unsigned: true,
                            ..Default::default()
                        });
                        to_add = false;
                    }
                    // dest = if src2 < src1 {1} else {0}
                    // dest = if dest == 1 {0} else {1}
                    Operator::LECompare => {
                        in_function.push(RiscvInstruction {
                            tt: SLT,
                            dest: *dest as i32,
                            src1: *src2 as i32,
                            src2: *src1 as i32,
                            ..Default::default()
                        });
                        in_function.push(RiscvInstruction {
                            tt: SLTI,
                            dest: *dest as i32,
                            src1: *dest as i32,
                            immediate: 1,
                            is_unsigned: true,
                            ..Default::default()
                        });
                        to_add = false;
                    }
                    // dest = if src1 < src2 {1} else {0}
                    // dest = if dest {0} else {1}
                    Operator::GECompare => {
                        in_function.push(RiscvInstruction {
                            tt: SLT,
                            dest: *dest as i32,
                            src1: *src1 as i32,
                            src2: *src2 as i32,
                            ..Default::default()
                        });
                        in_function.push(RiscvInstruction {
                            tt: SLT,
                            dest: *dest as i32,
                            src1: X0,
                            src2: *dest as i32,
                            is_unsigned: true,
                            ..Default::default()
                        });
                        to_add = false;
                    }
                    // dest = if src2 < src1 {1} else {0}
                    Operator::GTCompare => {
                        binary_instruction.src2 = *src1 as i32;
                        binary_instruction.src1 = *src2 as i32;
                        binary_instruction.tt = SLT;
                        to_add = false;
                    }
                    // dest = if src1 < src2 {1} else {0}
                    Operator::LTCompare => binary_instruction.tt = SLT,
                    Operator::Minus => binary_instruction.tt = SUB,
                    Operator::Plus => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = ADDI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => match self.constants_map.get(src1) {
                            Some(v) => {
                                binary_instruction.tt = ADDI;
                                binary_instruction.src1 = binary_instruction.src2;
                                binary_instruction.src2 = 0;
                                binary_instruction.immediate = *v as i32;
                            }
                            None => {
                                binary_instruction.tt = ADD;
                            }
                        },
                    },
                    Operator::Asterisk => binary_instruction.tt = MUL,
                    Operator::Slash => binary_instruction.tt = DIV,
                    Operator::XorOp => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = XORI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => match self.constants_map.get(src1) {
                            Some(v) => {
                                binary_instruction.tt = XORI;
                                binary_instruction.src1 = binary_instruction.src2;
                                binary_instruction.src2 = 0;
                                binary_instruction.immediate = *v as i32;
                            }
                            None => {
                                binary_instruction.tt = XOR;
                            }
                        },
                    },
                    Operator::AndOp => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = ANDI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => match self.constants_map.get(src1) {
                            Some(v) => {
                                binary_instruction.tt = ANDI;
                                binary_instruction.src1 = binary_instruction.src2;
                                binary_instruction.src2 = 0;
                                binary_instruction.immediate = *v as i32;
                            }
                            None => {
                                binary_instruction.tt = AND;
                            }
                        },
                    },
                    Operator::OrOp => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = ORI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => match self.constants_map.get(src1) {
                            Some(v) => {
                                binary_instruction.tt = ORI;
                                binary_instruction.src1 = binary_instruction.src2;
                                binary_instruction.src2 = 0;
                                binary_instruction.immediate = *v as i32;
                            }
                            None => {
                                binary_instruction.tt = OR;
                            }
                        },
                    },
                    Operator::Module => binary_instruction.tt = REM,
                    Operator::LShift => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = SLLI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => {
                            binary_instruction.tt = SLL;
                        }
                    },
                    Operator::RShift => match self.constants_map.get(src2) {
                        Some(v) => {
                            binary_instruction.tt = SRLI;
                            binary_instruction.src2 = 0;
                            binary_instruction.immediate = *v as i32;
                        }
                        None => {
                            binary_instruction.tt = SRL;
                        }
                    },
                    _ => panic!("Invalid binary operator {:#?}", tk),
                }
                if to_add {
                    in_function.push(binary_instruction);
                }
            }
            _ => panic!("Non valid node"),
        }
        return (in_function, post_function);
    }

    fn substitute_nodes(&self, in_function: &Vec<RiscvInstruction>, args: &Vec<TypeWrapper>) -> Vec<RiscvInstruction> {
        let mut result = in_function.clone();
        // Substitute the values of the argument registers with `ax`
        // This already takes into account the limitations on the maximum number of arguments
        for node in &mut result {
            for i in 0..args.len() {
                if i >= 8 {
                    break;
                }
                if node.src1 == i as i32 + 1 {
                    node.src1 = A0 - i as i32;
                }
                if node.src2 == i as i32 + 1 {
                    node.src2 = A0 - i as i32;
                }
            }
        }

        return result;
    }

    fn remove_load_constant(&self, mut nodes: Vec<RiscvInstruction>) -> Vec<RiscvInstruction> {
        let copy_nodes = nodes.clone();
        nodes.retain(|node| {
            if node.tt == ADDI {
                for n in &copy_nodes {
                    if n.src1 == node.dest || n.src2 == node.dest {
                        return true;
                    }
                }
                return false;
            }
            return true;
        });
        return nodes;
    }

    pub fn generate_code(&mut self, ir: &IrNode) -> u32 {
        let functions_list = if let Program(functions_list) = ir {
            functions_list
        } else {
            panic!("Provided node to `control_flow_removal` not of type Program")
        };

        for (i_function, function) in functions_list.iter().enumerate() {
            let mut in_function: Vec<RiscvInstruction> = vec![];
            let mut result: Vec<RiscvInstruction> = vec![];
            self.constants_map.clear();

            let (name, _, args, nodes) = if let FunctionDeclaration(name, tt, args, nodes) = function {
                (name, tt, args, nodes)
            } else {
                panic!("Provided node to `control_flow_removal` not of type FunctionDeclaration")
            };

            // Some variables are to be allocated on the stack. We obtain the size of the stack
            // activation required for the allocation and the offset of each variable on the stack
            let (ssa, stack_position) = self.get_alloc_stack_offset(nodes);

            // Get prelude and postlude of the function
            let mut pre_function = self.get_pre_function(&name, ssa, &args);
            let mut post_function = self.get_post_function(ssa);

            if name == "init" {
                self.global_stack_position = stack_position.clone();
            }

            for node in nodes {
                let (mut to_add_in, mut to_add_post) = self.convert_node(node, &stack_position, i_function as u32);
                in_function.append(&mut to_add_in);
                to_add_post.append(&mut post_function);
                post_function = to_add_post;
            }

            in_function = self.substitute_nodes(&in_function, &args);

            result.append(&mut pre_function);
            result.append(&mut in_function);
            if name != "init" {
                result.append(&mut post_function);
            }

            result = self.remove_load_constant(result);

            for elem in &result {
                print!("{}", elem.to_string());
            }
        }

        return 0;
    }

    pub fn get_alloc_stack_offset(&self, ir: &Vec<IrNode>) -> (u32, Vec<StackOffset>) {
        let mut result: Vec<StackOffset> = vec![];
        let mut current_offset = 0;
        let available_sizes = vec![4, 2, 1];

        for s in available_sizes {
            // Look for elements of size `s` (in 4, 2 and 1) and allocate them
            for node in ir {
                if let Alloc(tt, register, _, _, _, from_register, name) = node {
                    // Allocation of the arrays happen on top of the stack, not in the activation
                    // record
                    if tt.get_size() == s && !from_register {
                        result.push(StackOffset {
                            size: s,
                            reg: *register,
                            offset: -current_offset,
                            name: name.to_string(),
                        });
                        current_offset += s as i32;
                    }
                }
            }
        }

        // `ra` and `s0` are to be stored in the activation record
        current_offset += 8;

        // The stack grows by multiple of 16
        current_offset = if current_offset % 16 == 0 {
            current_offset
        } else {
            current_offset + (16 - current_offset % 16)
        };

        return (current_offset as u32, result);
    }
}
