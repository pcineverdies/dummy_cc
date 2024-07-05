/// enum RiscvInstructionType
///
/// List of all the instructions available on RV32IM, plus some pseudo instructions useful for
/// prototyping purposes. The instruction AUIPC is not included in the list, as it was not used in
/// the codegen
///
/// 6 5
/// 5 7
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum RiscvInstructionType {
    ADDI,  // add immediate
    SLTI,  // set less than immediate
    ANDI,  // and immediate
    ORI,   // or immediate
    XORI,  // xor immediate
    SLLI,  // shift left immediate
    SRLI,  // shift right immediate (either signed or unsigned)
    LUI,   // load upper immediate
    ADD,   // add
    SLT,   // set less than
    AND,   // and
    OR,    // or
    XOR,   // xor
    SLL,   // shift left
    SRL,   // shift right (either signed or unsigned)
    SUB,   // sub
    J,     // jump label
    JAL,   // jump and link
    BEQ,   // branch equal
    BNE,   // branch not equal
    BLT,   // branch less than
    BGE,   // branch greater equal
    LB,    // load byte
    LH,    // load half-word
    LW,    // load word
    SB,    // store byte
    SH,    // store half-word
    SW,    // store word
    DIV,   // division (either signed or unsigned)
    REM,   // remainder (either signed or unsigned)
    MUL,   // multiplication
    LABEL, // label
    #[default]
    NOP,
    LABELFUNCTION, // label function
}

use RiscvInstructionType::*;

impl RiscvInstructionType {
    /// RiscvInstructionType::to_string
    ///
    /// Get the name of the riscv instruction
    ///
    /// @return [String]: name of the instruction
    pub fn to_string(&self) -> String {
        match *self {
            ADDI => format!("addi"),
            SLTI => format!("slti"),
            ANDI => format!("andi"),
            ORI => format!("ori"),
            XORI => format!("xori"),
            SLLI => format!("slli"),
            SRLI => format!("srli"),
            LUI => format!("lui"),
            ADD => format!("add"),
            SLT => format!("slt"),
            AND => format!("and"),
            OR => format!("or"),
            XOR => format!("xor"),
            SLL => format!("sll"),
            SRL => format!("srl"),
            SUB => format!("sub"),
            J => format!("j"),
            JAL => format!("jal"),
            BEQ => format!("beq"),
            BNE => format!("bne"),
            BLT => format!("blt"),
            BGE => format!("bge"),
            LB => format!("lb"),
            LH => format!("lh"),
            LW => format!("lw"),
            SB => format!("sb"),
            SH => format!("sh"),
            SW => format!("sw"),
            DIV => format!("div"),
            REM => format!("rem"),
            MUL => format!("mul"),
            LABEL | LABELFUNCTION => format!(""),
            NOP => format!("nop"),
        }
    }
}

/// struct RiscvInstruction
///
/// Each instruction is represented using a struct having all the necessary fields. A struct is
/// preferred with respect to an enum due to the easiness in modifying the elements during the
/// codegen steps. Clearly, not all the fields are used for all the instructions
///
/// Registers are represented with a signed number. Negative values represent specific registers,
/// while positive registers are general purpose ones. If the field `register_allocated` is set,
/// than, the positive value of a register represents a physical register (limited), otherwise a
/// virtual register (unlimited)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct RiscvInstruction {
    pub tt: RiscvInstructionType, // Type of instruction
    pub dest: i32,
    pub src1: i32,
    pub src2: i32,
    pub immediate: i32,
    pub label: u32,
    pub label_function: u32,
    pub is_unsigned: bool,
    pub name: String,
    pub register_allocated: bool,
}

/// Set of constants used to represent some specific registers of the ISA. These registers are to
/// be set during codegen, and cannot be handled during register allocation.
pub const SP: i32 = -1; // stack pointer
pub const GP: i32 = -2; // global pointer
pub const FP: i32 = -3; // frame pointer
pub const RA: i32 = -4; // return address
pub const X0: i32 = -5; // zero constant
pub const A0: i32 = -6; // first function argument / return value
                        // as the system is on 32 bits and no value is larger than that, a1 is
                        // never used as return address

impl RiscvInstruction {
    /// RiscvInstruction::reg_to_string
    ///
    /// Get the name of a register. The specific registers are expressed with their names.
    /// Allocated register are expressed with `x` as prefix, otherwise with `v`
    /// Registers used for arguments are expressed with integers from -6 to -13, with names from
    /// `a0` to `a7`
    ///
    /// @in reg [i32]: register number
    /// @in allocated [bool]: is the register physical or virtual
    /// @return [String]: representation of the register
    pub fn reg_to_string(reg: i32, allocated: bool) -> String {
        match reg {
            SP => format!("sp"),
            GP => format!("gp"),
            FP => format!("s0"),
            RA => format!("ra"),
            X0 => format!("x0"),
            -6 => format!("a0"),
            -7 => format!("a1"),
            -8 => format!("a2"),
            -9 => format!("a3"),
            -10 => format!("a4"),
            -11 => format!("a5"),
            -12 => format!("a6"),
            -13 => format!("a7"),
            _ => {
                if allocated {
                    if reg <= 6 {
                        format!("t{}", reg)
                    } else {
                        format!("s{}", reg - 6)
                    }
                } else {
                    format!("r{}", reg)
                }
            }
        }
    }

    /// RiscvInstruction::to_string
    ///
    /// Transforms an instruction to string
    ///
    /// @return [String]: string version of the instruction
    pub fn to_string(&self) -> String {
        match self.tt {
            // Arithmetical instructions with immediate as argument
            ADDI | ANDI | ORI | XORI | SLLI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                self.immediate
            ),

            // Arithmetical instructions with immediate as argument and possible unsigned version
            SLTI => {
                let mut opcode = format!("\t{}", self.tt.to_string());
                if self.is_unsigned {
                    opcode += &"u";
                }
                format!(
                    "{}\t{}, {}, {}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                    RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                    self.immediate
                )
            }

            SRLI => {
                let mut opcode = format!("\t");
                if self.is_unsigned {
                    opcode += &"srli";
                } else {
                    opcode += &"srai";
                }
                format!(
                    "{}\t{}, {}, {}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                    RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                    self.immediate
                )
            }

            // Load upper immediate
            LUI => format!(
                "\t{}\t{}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                self.immediate,
            ),

            // Arithmetical instruction with two registers as arguments
            ADD | AND | OR | XOR | SLL | SUB | MUL => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated)
            ),

            // Arithmetical instruction with two registers as arguments and possible unsigned
            // version
            SLT | DIV | REM | SRL => {
                let mut opcode = format!("\t{}", self.tt.to_string());
                if self.is_unsigned {
                    opcode += &"u";
                }
                format!(
                    "{}\t{}, {}, {}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                    RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                    RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                )
            }

            // Jump instruction
            J => format!("\tjal\tx0, L_{}_{}\n", self.label_function, self.label),

            // Jump and link instruction, both with a register as destination or label
            JAL => {
                if self.src1 == 0 {
                    format!(
                        "\t{}\t{}, {}\n",
                        self.tt.to_string(),
                        RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                        self.name
                    )
                } else {
                    format!(
                        "\tjalr\t{}, {}, 0\n",
                        RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                        RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                    )
                }
            }

            // Branch to label comparing two registers
            BEQ | BNE => format!(
                "\t{}\t{}, {}, L_{}_{}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                self.label_function,
                self.label
            ),

            // Branch to label comparing two registers, possible unsigned version
            BLT | BGE => {
                let mut opcode = format!("\t{}", self.tt.to_string());
                if !self.is_unsigned {
                    opcode += &"u";
                }
                format!(
                    "{}\t{}, {}, L_{}_{}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                    RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                    self.label_function,
                    self.label
                )
            }

            // Load instruction
            LH | LW | LB => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated)
            ),

            // Store instruction
            SB | SH | SW => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
            ),
            // Label
            LABEL => format!("L_{}_{}:\n", self.label_function, self.label),
            // NOP
            NOP => format!("\t{}\n", self.tt.to_string()),
            // Label function
            LABELFUNCTION => format!("\n{}:\n", self.name),
        }
    }
}
