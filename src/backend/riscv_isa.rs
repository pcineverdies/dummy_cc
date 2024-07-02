#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum RiscvInstructionType {
    ADDI,
    SLTI,
    ANDI,
    ORI,
    XORI,
    SLLI,
    SRLI,
    LUI,
    ADD,
    SLT,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SUB,
    J,
    JAL,
    BEQ,
    BNE,
    BLT,
    BGE,
    LB,
    LH,
    LW,
    SB,
    SH,
    SW,
    DIV,
    REM,
    MUL,
    LABEL,
    #[default]
    NOP,
    LABELFUNCTION,
}

use RiscvInstructionType::*;

impl RiscvInstructionType {
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

pub const SP: i32 = -1;
pub const GP: i32 = -2;
pub const FP: i32 = -3;
pub const RA: i32 = -4;
pub const X0: i32 = -5;
pub const A0: i32 = -6;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct RiscvInstruction {
    pub tt: RiscvInstructionType,
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

impl RiscvInstruction {
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
                    format!("x{}", reg)
                } else {
                    format!("r{}", reg)
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self.tt {
            ADDI | ANDI | ORI | XORI | SLLI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                self.immediate
            ),
            SLTI | SRLI => {
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
            LUI => format!(
                "\t{}\t{}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                self.immediate,
            ),
            ADD | AND | OR | XOR | SLL | SUB => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated)
            ),
            SLT | DIV | REM | MUL | SRL => {
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
            J => format!("\tjal\tx0, L_{}_{}\n", self.label_function, self.label),
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
            BEQ | BNE => format!(
                "\t{}\t{}, {}, L_{}_{}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                self.label_function,
                self.label
            ),
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
            LH | LW | LB => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest, self.register_allocated),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated)
            ),
            SB | SH | SW => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src2, self.register_allocated),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1, self.register_allocated),
            ),
            LABEL => format!("L_{}_{}:\n", self.label_function, self.label),
            NOP => format!("\t{}\n", self.tt.to_string()),
            LABELFUNCTION => format!("\n{}:\n", self.name),
        }
    }
}
