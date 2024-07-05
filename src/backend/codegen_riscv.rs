use std::collections::HashMap;

use crate::ast::type_wrapper::TypeWrapper;
use crate::backend::riscv_isa::{RiscvInstruction, RiscvInstructionType, A0, FP, GP, RA, SP, X0};
use crate::lexer::token::Operator;
use crate::lirgen::irnode::{CompareType, IrNode};

use IrNode::*;
use RiscvInstructionType::*;

const SP_INIT_VALUE: i32 = 0x00010000;

/// struct StackOffset
///
/// Struct to store the information of the variables allocated on the stack. At the beginning of
/// a function, we go over all the `alloc` nodes of non-array variables in order to store in the
/// activation block of the function. For each variables we store its size, the register which
/// stores it, its offset with respect to the frame pointer and the name of the variable
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StackOffset {
    size: u32,    // size of the variable
    reg: u32,     // register storing the address to the variable
    offset: i32,  // offset of the variable with respect to the frame pointer
    name: String, // name of the variable
}

/// struct Codegen
///
/// Struct to handle the codegen process.
///
/// Struct to handle the codegen process.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Codegen {
    // Sometimes a new register is required. This variable stores the next value to be used
    current_new_register: i32,

    // Map of the registers containing constant values. This is used to implement
    // the immediate instructions
    constants_map: HashMap<u32, u32>,

    // Stack used by the `init` function, containing the global variables
    global_stack_position: Vec<StackOffset>,
}

impl Codegen {
    /// Codegen::new
    ///
    /// Create a new codegen object.
    pub fn new() -> Codegen {
        Codegen {
            // The value of current_new_register is initialized to a very large value, so that it
            // will not collide with the registers used by the linear IR
            current_new_register: 2_i32.pow(15),
            // Initially empty
            constants_map: HashMap::new(),
            // Initially empty
            global_stack_position: vec![],
        }
    }

    /// Codegen::get_new_register
    ///
    /// Get a new register
    ///
    /// @return [i32]: value of register to use
    fn get_new_register(&mut self) -> i32 {
        let result = self.current_new_register;
        self.current_new_register += 1;
        return result;
    }

    /// Codegen::get_pre_function
    ///
    /// Each function has a preamble, containing some instruction to prepare the activation record
    /// of the function
    ///
    /// @in [&String]: name of the function
    /// @in [u32]: size of the activation record (which has to be a multiple of 16)
    /// @in [&Vec<TypeWrapper>]: list of arguments of the function
    /// @return [Vec<RiscvInstruction>]: list of instructions ofr the preamble
    fn get_pre_function(&self, name: &String, ssa: u32, args: &Vec<TypeWrapper>) -> Vec<RiscvInstruction> {
        let mut pre_function: Vec<RiscvInstruction> = vec![];

        if ssa % 16 != 0 {
            panic!("Increments of SP must be multiple of 16; {} provided", ssa);
        }

        // Label of the function
        pre_function.push(RiscvInstruction {
            tt: LABELFUNCTION,
            name: name.to_string(),
            ..Default::default()
        });

        // Decrement of SP
        pre_function.push(RiscvInstruction {
            tt: ADDI,
            dest: SP,
            src1: SP,
            immediate: -(ssa as i32),
            ..Default::default()
        });

        // Store the return address on the stack
        pre_function.push(RiscvInstruction {
            tt: SW,
            src1: SP,
            src2: RA,
            immediate: 0 as i32,
            ..Default::default()
        });

        // Store the previous frame pointer on the stack
        pre_function.push(RiscvInstruction {
            tt: SW,
            src1: SP,
            src2: FP,
            immediate: 4 as i32,
            ..Default::default()
        });

        // the previous value of the stack is to be used as frame pointer (base value of the
        // activation record)
        pre_function.push(RiscvInstruction {
            tt: ADDI,
            dest: FP,
            src1: SP,
            immediate: (ssa as i32),
            ..Default::default()
        });

        // If the function has more than 8 arguments, the additional arguments are provided on the
        // stack, before the new FP. These instructions load those values in the appropriate
        // virtual registers.
        if args.len() > 8 {
            todo!();
        }

        // If we hare handling the init function, we have to initialize the stack pointer to a
        // picked value, and then use the frame pointer as global pointer, since global variables
        // are initialized in the `init`, stored on the stack and referenced from there
        if name == "init" {
            pre_function.insert(
                1,
                RiscvInstruction {
                    tt: LUI,
                    dest: SP,
                    immediate: SP_INIT_VALUE >> 12,
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

    /// Codegen::get_post_function
    ///
    /// Each function has an epilogue, in which all the modified variables are brought back to the
    /// previous values.
    ///
    /// @in ssa[u32]: size of the activation record
    /// @return [Vec<RiscvInstruction>]: list of instructions to use
    fn get_post_function(&self, ssa: u32) -> Vec<RiscvInstruction> {
        let mut post_function: Vec<RiscvInstruction> = vec![];

        // Restore return address
        post_function.push(RiscvInstruction {
            tt: LW,
            dest: RA,
            src1: SP,
            immediate: 0 as i32,
            ..Default::default()
        });

        // Restore frame pointer
        post_function.push(RiscvInstruction {
            tt: LW,
            dest: FP,
            src1: SP,
            immediate: 4 as i32,
            ..Default::default()
        });

        // Put the stack pointer to the previous value
        post_function.push(RiscvInstruction {
            tt: ADDI,
            dest: SP,
            src1: SP,
            immediate: (ssa as i32),
            ..Default::default()
        });

        // Jump to the return address without saving the return address
        post_function.push(RiscvInstruction {
            tt: JAL,
            dest: X0,
            src1: RA,
            ..Default::default()
        });

        return post_function;
    }

    /// Codegen::convert_node
    ///
    /// Convert an IR node to a list of RV32IM instructions to handle it
    ///
    /// @in node [&IrNode]: node to translate
    /// @in stack_position [&Vec<StackOffset>]: list of variables stored in the stack
    /// @in i_function [u32]: function index used to create labels
    /// @return [(Vec<RiscvInstruction>, Vec<RiscvInstruction>)]: The first element is the list of
    /// instructions which represents the functions. The second element is the list of instructions
    /// to be added to the prologue of the function (mainly in order to restore the correct SP
    /// value).
    fn convert_node(&mut self, node: &IrNode, stack_position: &Vec<StackOffset>, i_function: u32) -> (Vec<RiscvInstruction>, Vec<RiscvInstruction>) {
        let mut in_function: Vec<RiscvInstruction> = vec![];
        let mut post_function: Vec<RiscvInstruction> = vec![];
        match node {
            // The return node is at the end of a function, and it consists in moving the value of
            // the register storing the return value into `a0` before running the epilogue of the
            // function
            Return(_, src) => {
                if *src != 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: A0,
                        src1: *src as i32,
                        ..Default::default()
                    });
                }
                in_function.push(RiscvInstruction {
                    tt: J,
                    label_function: i_function as u32,
                    label: 0,
                    ..Default::default()
                });
            }

            // An Alloc node might represent the allocation of an array or of a variable. In the
            // fist case, the space for the pointer to the array is already in the activation
            // record, and the array is to be saved on top of the stack. Otherwise, the space of
            // the stack is to be initialized
            Alloc(tt, dest, src, _, size, from_reg, ..) => {
                if *from_reg {
                    // A new register is used to store the amount for which the stack is to be
                    // modified. This must be a multiple of 15, while the size of the vector is
                    // arbitrary.
                    let new_register = self.get_new_register();

                    // The size of the register is stored in `vx`, while this value has to be
                    // multiplied by the size in bytes of the content
                    in_function.push(RiscvInstruction {
                        tt: SLLI,
                        dest: new_register,
                        src1: *size as i32,
                        immediate: tt.get_size() as i32 / 2,
                        ..Default::default()
                    });
                    // Starting from register `vx`, we can obtain the closest upper multiple of 16
                    // by adding 15 to it and masking the last 4 bits.
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: new_register,
                        src1: new_register,
                        immediate: 15 as i32,
                        ..Default::default()
                    });
                    in_function.push(RiscvInstruction {
                        tt: ANDI,
                        dest: new_register,
                        src1: new_register,
                        immediate: (0xfffffff0 as u32) as i32,
                        ..Default::default()
                    });
                    // Space for the vector is created
                    in_function.push(RiscvInstruction {
                        tt: SUB,
                        dest: SP,
                        src1: SP,
                        src2: new_register,
                        ..Default::default()
                    });
                    // We store the pointer to the vector in the `dest` register
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: *dest as i32,
                        src1: SP,
                        immediate: 0,
                        ..Default::default()
                    });
                    // At the beginning of the epilogue, the SP has to be increased again in order
                    // to compensate for the previous size.
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

                // If the source value is not set, in this case the space for the variable is not
                // initialized. This is also the case for an array declaration
                if *src == 0 {
                    return (in_function, post_function);
                }

                // A store instruction is required to initialize the space on the stack
                let mut store_instruction = RiscvInstruction { ..Default::default() };
                // Choose the right size of the store instruction
                store_instruction.tt = match tt.get_size() {
                    4 => SW,
                    2 => SH,
                    _ => SB,
                };
                // Source of the store is src2
                store_instruction.src2 = *src as i32;
                // If the value to initialize is on the stack, then we refer to it using the `fp`
                // pointer and its relative offset. Otherwise, we use the register containing the
                // address
                for elem in stack_position {
                    if elem.reg == *dest {
                        store_instruction.src1 = FP;
                        store_instruction.immediate = elem.offset as i32;
                        in_function.push(store_instruction);
                        in_function.push(RiscvInstruction {
                            tt: ADDI,
                            dest: *dest as i32,
                            src1: FP,
                            immediate: elem.offset as i32,
                            ..Default::default()
                        });
                        return (in_function, post_function);
                    }
                }
                // If the register is not found on the stack, then use the pointer register to
                // perform the store, with offset 0
                store_instruction.src1 = *dest as i32;
                in_function.push(store_instruction);
            }
            MovC(_, dest, src) => {
                // If the constant is larger than 2**12, a LUI is required
                if *src > (1 << 12) {
                    in_function.push(RiscvInstruction {
                        tt: LUI,
                        dest: *dest as i32,
                        immediate: (src >> 12) as i32,
                        ..Default::default()
                    });
                } else {
                    // Otherwise, add the register to the list of registers storing constants, so that
                    // if it is found in an arithmetic instruction it can be substituted with the
                    // correspondent constant
                    self.constants_map.insert(*dest, *src);
                }
                // Move the constant value in the register
                in_function.push(RiscvInstruction {
                    tt: ADDI,
                    dest: *dest as i32,
                    src1: X0,
                    immediate: (src % (1 << 12)) as i32,
                    ..Default::default()
                });
            }

            // A cast operation is done by first resizing the register from M to N bits, and then
            // possibly doing a signed extension. The first operation can be done with a bit mask.
            // The second operation can be done with a series of a left shift and signed right
            // shift
            Cast(ttd, _, dest, src) => {
                // Destination register is different from 4
                // We clear the upper N bits of the register, which are 24 (size == 1) or 16 (size == 2)
                let and_mask = match ttd.get_size() {
                    1 => 0xff,
                    2 => 0xffff,
                    _ => -1,
                };
                in_function.push(RiscvInstruction {
                    tt: ANDI,
                    dest: *dest as i32,
                    src1: *src as i32,
                    immediate: and_mask as i32,
                    ..Default::default()
                });
                // If the destination is signed and different form i32, we shift left until we have
                // the important bits on the leftmost side, and then shift right signed to adjust
                // the sign
                if ttd.is_signed() && ttd.get_size() != 4 {
                    let shift_size = 32 - ttd.get_size() * 8;
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
            // A store instruction has a destination which is either a relative point to the fp or
            // the content of a register
            Store(tt, dest, src) => {
                let mut store_instruction = RiscvInstruction { ..Default::default() };
                // Pick the size of the store
                store_instruction.tt = match tt.get_size() {
                    4 => SW,
                    2 => SH,
                    _ => SB,
                };
                // Source register is fixed
                store_instruction.src2 = *src as i32;
                store_instruction.src1 = *dest as i32;
                // See if destination comes from the stack
                for elem in stack_position {
                    if elem.reg == *dest {
                        store_instruction.src1 = FP;
                        store_instruction.immediate = elem.offset;
                        in_function.push(store_instruction);
                        return (in_function, post_function);
                    }
                }
                in_function.push(store_instruction);
            }
            // Load address to global variable. They are all referenced from an offset to GP, thus
            // the destination gets the pointer to that variable
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
            // Load having the pointer of the variable to load in a register
            LoadR(tt, dest, src) => {
                let mut load_instruction = RiscvInstruction { ..Default::default() };
                // Size of the load
                load_instruction.tt = match tt.get_size() {
                    4 => LW,
                    2 => LH,
                    _ => LB,
                };
                // Destination is fixed
                load_instruction.dest = *dest as i32;
                // The address might also be on the stack, if we are referring to a local variable
                for elem in stack_position {
                    if elem.reg == *src {
                        load_instruction.src1 = FP;
                        load_instruction.immediate = elem.offset as i32;
                        in_function.push(load_instruction);
                        return (in_function, post_function);
                    }
                }
                load_instruction.src1 = *src as i32;
                in_function.push(load_instruction);
            }
            // Add a label
            Label(s) => in_function.push(RiscvInstruction {
                tt: LABEL,
                label: *s,
                label_function: i_function as u32,
                ..Default::default()
            }),
            // Call to a function, which requires to handle the load of the arguments in the proper
            // registers, and possibly handling the extra arguments with the stack
            Call(name, _, arguments, ret) => {
                let extra_arguments: i32 = arguments.len() as i32 - 8;
                let extra_space: i32 = ((extra_arguments * 4) + 15) & 0xffffff0 as i32;
                for i in 0..arguments.len() {
                    // For the first 8 arguments, we move them in the register `Ai` (0-indexed)
                    if i < 8 {
                        in_function.push(RiscvInstruction {
                            tt: ADDI,
                            dest: A0 - i as i32,
                            src1: arguments[i] as i32,
                            immediate: 0,
                            ..Default::default()
                        });
                    // Otherwise, we push them on the stack
                    } else {
                        todo!();
                    }
                }
                // Move the SP if required
                if extra_arguments > 0 {
                    todo!();
                }
                // Add a jump to the function
                in_function.push(RiscvInstruction {
                    tt: JAL,
                    dest: RA,
                    name: name.to_string(),
                    ..Default::default()
                });
                // Mov the return value to the correct register
                if *ret != 0 {
                    in_function.push(RiscvInstruction {
                        tt: ADDI,
                        dest: *ret as i32,
                        src1: A0,
                        immediate: 0,
                        ..Default::default()
                    });
                }
                // Put the stack value back
                if extra_arguments > 0 {
                    todo!();
                }
            }
            // Handle a branch
            Branch(ct, tt, src1, src2, name) => {
                let mut branch_instruction = RiscvInstruction { ..Default::default() };
                branch_instruction.label = *name;
                branch_instruction.label_function = i_function as u32;
                branch_instruction.src1 = *src1 as i32;
                branch_instruction.src2 = *src2 as i32;
                match ct {
                    // Always is just J
                    CompareType::Always => {
                        branch_instruction.tt = J;
                    }
                    // GT is implemented by inverting the operands and using LT
                    CompareType::GT => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BLT;
                        branch_instruction.src2 = *src1 as i32;
                        branch_instruction.src1 = *src2 as i32;
                    }
                    // GE is already present
                    CompareType::GE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BGE;
                    }
                    // LT is already present
                    CompareType::LT => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BLT;
                    }
                    // LE is implemented by inverting GE
                    CompareType::LE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BGE;
                        branch_instruction.src2 = *src1 as i32;
                        branch_instruction.src1 = *src2 as i32;
                    }
                    // EQ is already implemented
                    CompareType::EQ => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BEQ;
                    }
                    // NE is already implemented
                    CompareType::NE => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BNE;
                    }
                    // S can be implemented as a `not equal` with zero
                    CompareType::S => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BNE;
                        branch_instruction.src2 = X0;
                    }
                    // NS can be implemented as a `equal` with zero
                    CompareType::NS => {
                        branch_instruction.is_unsigned = !tt.is_signed();
                        branch_instruction.tt = BEQ;
                        branch_instruction.src2 = X0;
                    }
                }
                in_function.push(branch_instruction);
            }
            // Unary operation
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
            // Binary operation
            Binary(tk, tt, dest, src1, src2) => {
                let mut binary_instruction = RiscvInstruction { ..Default::default() };
                binary_instruction.dest = *dest as i32;
                binary_instruction.src1 = *src1 as i32;
                binary_instruction.src2 = *src2 as i32;
                binary_instruction.is_unsigned = !tt.is_signed();
                let mut to_add = true;
                match tk {
                    // vx == vy can be implemented by computing the subtraction between the two
                    // operands and setting the destination to 1 if the result is 0
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
                    // vx == vy can be implemented by computing the subtraction between the two
                    // operands and setting the destination to 1 if the result is not 0
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
                    // First we set dest if src1 > src2, then we do the opposite of the result
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
                    // First we set dest if src1 < src2, then we do the opposite of the result
                    // dest = if src2 < src1 {1} else {0}
                    // dest = if dest == 1 {0} else {1}
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
                    // First we set dest if src2 < src1, then we do the opposite of the result
                    // dest = if src2 < src1 {1} else {0}
                    // dest = if dest == 1 {0} else {1}
                    Operator::GTCompare => {
                        binary_instruction.src2 = *src1 as i32;
                        binary_instruction.src1 = *src2 as i32;
                        binary_instruction.tt = SLT;
                    }
                    // dest = if src1 < src2 {1} else {0}
                    Operator::LTCompare => binary_instruction.tt = SLT,
                    Operator::Minus => binary_instruction.tt = SUB,
                    // Implement the plus operator, by checking if either the left operand or the
                    // right operand is a constant
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
                    // Multiplication
                    Operator::Asterisk => binary_instruction.tt = MUL,
                    // Division
                    Operator::Slash => binary_instruction.tt = DIV,
                    // Implement the xor operator, by checking if either the left operand or the
                    // right operand is a constant
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
                    // Implement the and operator, by checking if either the left operand or the
                    // right operand is a constant
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
                    // Implement the or operator, by checking if either the left operand or the
                    // right operand is a constant
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
                    // Remainder
                    Operator::Module => binary_instruction.tt = REM,
                    // Left shift (right operand might be immediate)
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
                    // Right shift (right operand might be immediate)
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

    /// Codegen::substitute_argument_registers
    ///
    /// The registers associated to the input values of the function should be changed to `ax`
    ///
    /// @in [&Vec<RiscvInstruction>]: List of function nodes
    /// @in [&Vec<TypeWrapper>]: List of arguments to the function
    /// @return [Vec<RiscvInstruction>]: List of modified functions
    fn substitute_argument_registers(&self, mut in_function: Vec<RiscvInstruction>, args: &Vec<TypeWrapper>) -> Vec<RiscvInstruction> {
        // Substitute the values of the argument registers with `ax`
        // This already takes into account the limitations on the maximum number of arguments
        for node in &mut in_function {
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

        return in_function;
    }

    /// Codegen::remove_load_constant
    ///
    /// If a constant was loaded in a register, then the register might be substituted with a
    /// constant if the instruction support the `immediate` version. Due to this, some loads of
    /// constants on a register might be useless. They are all removed, freeing virtual registers
    ///
    /// @in [&Vec<RiscvInstruction>]: List of function nodes
    /// @in [&Vec<TypeWrapper>]: List of arguments to the function
    /// @return [Vec<RiscvInstruction>]: List of modified functions
    fn remove_load_constant(&self, mut nodes: Vec<RiscvInstruction>) -> Vec<RiscvInstruction> {
        let copy_nodes = nodes.clone();
        nodes.retain(|node| {
            if node.tt == ADDI && node.src1 == X0 && node.dest > 0 {
                // Keep the register if there is a node having that same register as source
                for n in &copy_nodes {
                    if n.src1 == node.dest || n.src2 == node.dest {
                        return true;
                    }
                }
                // If none was found, remove it
                return false;
            }
            // Keep all the other instructions
            return true;
        });
        return nodes;
    }

    /// Codegen::generate_code
    ///
    /// Starting from the list of instruction, transform it into assembly code
    ///
    /// @in ir [&IrNode]: input linear IR of the program
    /// @return [Vec<RiscvInstruction>]: list of instructions
    pub fn generate_code(&mut self, ir: &IrNode) -> Vec<RiscvInstruction> {
        // Vector containing the result of the instruction
        let mut code: Vec<RiscvInstruction> = vec![];

        // We expect the input IrNode to be a Program type, with the list of functions
        let functions_list = if let Program(functions_list) = ir {
            functions_list
        } else {
            panic!("Provided node to `control_flow_removal` not of type Program")
        };

        // For each function
        for (i_function, function) in functions_list.iter().enumerate() {
            let mut in_function: Vec<RiscvInstruction> = vec![];
            let mut result: Vec<RiscvInstruction> = vec![];
            self.constants_map.clear();

            // We expect the element to be a function declaration
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

            // If the function is `init`, then its stack is used as global stack
            if name == "init" {
                self.global_stack_position = stack_position.clone();
            }

            // Convert each node
            for node in nodes {
                let (mut to_add_in, mut to_add_post) = self.convert_node(node, &stack_position, i_function as u32);
                in_function.append(&mut to_add_in);
                to_add_post.append(&mut post_function);
                post_function = to_add_post;
            }

            // Change the arguments
            in_function = self.substitute_argument_registers(in_function, &args);

            // Create the functions by using pre_function and in_function
            result.append(&mut pre_function);
            result.append(&mut in_function);
            if name != "init" {
                post_function.insert(
                    0,
                    RiscvInstruction {
                        tt: LABEL,
                        label_function: i_function as u32,
                        label: 0,
                        ..Default::default()
                    },
                );
                result.append(&mut post_function);
            }

            result = self.remove_load_constant(result);

            // println!("\n---------- PRE ALLOCATION -----------");
            // for elem in &result {
            //     print!("{}", elem.to_string());
            // }
            // println!("\n---------- POST ALLOCATION -----------");
            result = self.register_allocation(result);

            for elem in &result {
                print!("{}", elem.to_string());
            }

            code.append(&mut result);
        }

        return code;
    }

    fn find_usage_register(&self, instructions: &Vec<RiscvInstruction>, starting_point: u32, target: i32) -> bool {
        let mut labels_found: Vec<u32> = vec![];
        let mut labels_to_analyze: Vec<(u32, usize)> = vec![];

        for i in starting_point as usize..instructions.len() {
            let instr = instructions[i].clone();
            if instr.tt == LABEL {
                labels_found.push(instr.label);
            }
            if instr.src1 == target || instr.src2 == target {
                return true;
            }
            if instr.dest == target {
                return false;
            }
            if instr.label > 0 {
                labels_to_analyze.push((instr.label, i));
            }
        }

        while labels_to_analyze.len() != 0 {
            let current_label = labels_to_analyze.pop().unwrap();
            if labels_found.contains(&current_label.0) {
                continue;
            }
            labels_found.push(current_label.0);

            let mut found = false;
            for i in 0..instructions.len() {
                let instr = instructions[i].clone();
                if instr.tt == LABEL && instr.label == current_label.0 {
                    found = true;
                    continue;
                }
                if !found {
                    continue;
                }
                let instr = instructions[i].clone();
                if instr.tt == LABEL && instr.label > 0 && labels_found.contains(&instr.label) {
                    break;
                }
                if instr.dest == target {
                    return false;
                }
                if instr.src1 == target || instr.src2 == target {
                    return true;
                }
                if instr.label > 0 && instr.tt == LABEL {
                    labels_found.push(instr.label);
                } else if instr.label > 0 {
                    labels_to_analyze.push((instr.label, i));
                }
            }
        }
        return false;
    }

    fn register_allocation(&self, instructions: Vec<RiscvInstruction>) -> Vec<RiscvInstruction> {
        let mut result: Vec<RiscvInstruction> = vec![];
        let mut virtual_register_allocation: HashMap<i32, i32> = HashMap::new();
        let mut is_register_used: Vec<(bool, bool, i32)> = vec![(false, false, 0); 18];

        for i in 0..instructions.len() {
            let mut instr = instructions[i].clone();
            if instr.tt == LABEL {
                for j in 0..is_register_used.len() {
                    if is_register_used[j].0 {
                        if !self.find_usage_register(&instructions, i as u32 + 1, is_register_used[j].2) {
                            is_register_used[j].0 = false;
                        }
                    }
                }
            }
            if instr.src1 > 0 {
                let virtual_value = instr.src1;
                match virtual_register_allocation.get(&virtual_value) {
                    Some(reg) => instr.src1 = *reg as i32,
                    None => panic!("Virtual register {} has no associated physical register", instr.src1),
                }
                if !self.find_usage_register(&instructions, i as u32 + 1, virtual_value) {
                    is_register_used[instr.src1 as usize].0 = false;
                }
            }
            if instr.src2 > 0 {
                let virtual_value = instr.src2;
                match virtual_register_allocation.get(&virtual_value) {
                    Some(reg) => instr.src2 = *reg as i32,
                    None => panic!("Virtual register {} has no associated physical register", instr.src2),
                }
                if !self.find_usage_register(&instructions, i as u32 + 1, virtual_value) {
                    is_register_used[instr.src2 as usize].0 = false;
                }
            }
            if instr.dest > 0 {
                let virtual_value = instr.dest;
                match virtual_register_allocation.get(&instr.dest) {
                    Some(reg) => {
                        instr.dest = *reg as i32;
                    }
                    None => {
                        let mut register_to_use: i32 = -1;
                        for i in 0..is_register_used.len() {
                            if !is_register_used[i].0 {
                                register_to_use = i as i32;
                                is_register_used[i] = (true, true, instr.dest);
                                virtual_register_allocation.insert(instr.dest, register_to_use);
                                instr.dest = register_to_use;
                                break;
                            }
                        }
                        if register_to_use == -1 {
                            panic!("no free registers");
                        }
                    }
                }
                if !self.find_usage_register(&instructions, i as u32 + 1, virtual_value) {
                    is_register_used[instr.dest as usize].0 = false;
                }
            }
            instr.register_allocated = true;

            // Store temporary registers currently in use
            if instr.tt == JAL && instr.src1 == 0 {
                for i in 0..=6 {
                    if is_register_used[i].0 {
                        result.push(RiscvInstruction {
                            tt: SW,
                            src1: FP,
                            src2: i as i32,
                            immediate: -4 - i as i32 * 4,
                            register_allocated: true,
                            ..Default::default()
                        });
                    }
                }
                result.push(instr);
                for i in 0..=6 {
                    if is_register_used[i].0 {
                        result.push(RiscvInstruction {
                            tt: LW,
                            dest: i as i32,
                            src1: FP,
                            immediate: -4 - i as i32 * 4,
                            register_allocated: true,
                            ..Default::default()
                        });
                    }
                }
            } else {
                result.push(instr);
            }
        }

        for i in 7..is_register_used.len() {
            if is_register_used[i].1 {
                result.insert(
                    1,
                    RiscvInstruction {
                        tt: SW,
                        src1: SP,
                        src2: i as i32,
                        immediate: -4 - i as i32 * 4,
                        register_allocated: true,
                        ..Default::default()
                    },
                );
                result.insert(
                    result.len() - 1,
                    RiscvInstruction {
                        tt: LW,
                        dest: i as i32,
                        src1: SP,
                        immediate: -4 - i as i32 * 4,
                        register_allocated: true,
                        ..Default::default()
                    },
                );
            }
        }

        return result;
    }

    fn get_alloc_stack_offset(&self, ir: &Vec<IrNode>) -> (u32, Vec<StackOffset>) {
        let mut result: Vec<StackOffset> = vec![];
        let mut current_offset = 76;
        // Leave the space to store the temporary registers before calling functions
        let mut ssa = 18 * 4;
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
                        ssa += s as i32;
                    }
                }
            }
        }

        // `ra` and `s0` are to be stored in the activation record
        ssa += 8;

        // The stack grows by multiple of 16
        ssa = (ssa + 15) & -16;

        return (ssa as u32, result);
    }
}
