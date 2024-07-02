use crate::backend::riscv_isa::{RiscvInstruction, RiscvInstructionType};
use crate::lirgen::irnode::{CompareType, IrNode};

use IrNode::*;
use RiscvInstructionType::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StackOffset {
    size: u32,
    reg: u32,
    offset: u32,
}

const SP: i32 = -1;
const GP: i32 = -2;
const FP: i32 = -3;
const RA: i32 = -4;
const X0: i32 = -5;
const A0: i32 = -6;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Codegen {}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {}
    }

    #[rustfmt::skip]
    pub fn generate_code(&mut self, ir: &IrNode) -> u32 {
        let functions_list = if let Program(functions_list) = ir {
            functions_list
        } else {
            panic!("Provided node to `control_flow_removal` not of type Program")
        };

        for (i_function, function) in functions_list.iter().enumerate() {

            let mut pre_function : Vec<RiscvInstruction> = vec![];
            let mut post_function : Vec<RiscvInstruction> = vec![];
            let mut in_function : Vec<RiscvInstruction> = vec![];
            let mut result : Vec<RiscvInstruction> = vec![];

            let (name, arg_types, args, nodes) = if let FunctionDeclaration(name, tt, args, nodes) = function {
                (name, tt, args, nodes)
            } else {
                panic!("Provided node to `control_flow_removal` not of type FunctionDeclaration")
            };

            // Some variables are to be allocated on the stack. We obtain the size of the stack
            // activation required for the allocation and the offset of each variable on the stack
            let (mut ssa, stack_position) = self.get_alloc_stack_offset(nodes);

            // `ra` and `s0` are to be stored in the activation record
            ssa += 8;

            // The stack grows by multiple of 16
            ssa = if ssa % 16 == 0 { ssa } else { ssa + (16 - ssa % 16) };

            // Create the space for the actication record
            pre_function.push(RiscvInstruction{tt:ADDI, dest: SP, src1: SP, immediate: -(ssa as i32), ..Default::default()});
            // Store RA in the activatino record
            pre_function.push(RiscvInstruction{tt:SW, src1: RA, src2: SP, immediate: ssa as i32 - 4, ..Default::default()});
            // Store FP in the activation record
            pre_function.push(RiscvInstruction{tt:SW, src1: FP, src2: SP, immediate: ssa as i32 - 8, ..Default::default()});
            // Use FP as frame pointer (pointer to the start of the activation record)
            pre_function.push(RiscvInstruction{tt:ADDI, dest: FP, src1: SP, immediate:  (ssa as i32), ..Default::default()});
            // Restore RA
            post_function.push(RiscvInstruction{tt:LW, dest: RA, src1: SP, immediate: ssa as i32 - 4, ..Default::default()});
            // Restore FP
            post_function.push(RiscvInstruction{tt:LW, dest: FP, src1: SP, immediate: ssa as i32 - 8, ..Default::default()});
            // Get the previous value of SP
            post_function.push(RiscvInstruction{tt:ADDI, dest: SP, src1: SP, immediate:  (ssa as i32), ..Default::default()});
            // Return
            post_function.push(RiscvInstruction{tt:JALR, dest: X0, src1: RA, ..Default::default()});

            for node in nodes {
                match node {
                    Return(tt, src) => {
                        if *src != 0 {
                            in_function.push(RiscvInstruction{tt:ADDI, dest: A0, src1: *src as i32, ..Default::default()});
                        }
                    },
                    Alloc(tt, dest, src, is_global, size, from_reg) => {},
                    MovC(tt, dest, src) => {},
                    Cast(ttd, tts, dest, src) => {},
                    Store(tt, dest, src) => {},
                    LoadA(tt, dest, src) => {},
                    LoadR(tt, dest, src) => {},
                    Label(s) => {},
                    Call(name, tt, arguments, ret) => {}
                    Branch(ct, tt, src1, src2, name) => {},
                    Unary(tt, tk, dest, src) => {},
                    Binary(tk, tt, dest, src1, src2) => {},
                    _ => panic!("Non valid node"),

                }
            }

            result.append(&mut pre_function);
            result.append(&mut in_function);
            result.append(&mut post_function);

            println!("--- function {} ----", name);

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
                if let Alloc(tt, register, _, _, _, from_register) = node {
                    // Allocation of the arrays happen on top of the stack, not in the activation
                    // record
                    if tt.get_size() == s && !from_register {
                        result.push(StackOffset {
                            size: s,
                            reg: *register,
                            offset: current_offset,
                        });
                        current_offset += s;
                    }
                }
            }
        }

        return (current_offset, result);
    }
}
