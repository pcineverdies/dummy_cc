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
    AUIPC,
    ADD,
    SLT,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SUB,
    J,
    JALR,
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
            AUIPC => format!("auipc"),
            ADD => format!("add"),
            SLT => format!("slt"),
            AND => format!("and"),
            OR => format!("or"),
            XOR => format!("xor"),
            SLL => format!("sll"),
            SRL => format!("srl"),
            SUB => format!("sub"),
            J => format!("j"),
            JALR => format!("jalr"),
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
}

impl RiscvInstruction {
    pub fn reg_to_string(reg: i32) -> String {
        match reg {
            -1 => format!("sp"),
            -2 => format!("gp"),
            -3 => format!("fp"),
            -4 => format!("ra"),
            -5 => format!("x0"),
            -6 => format!("a0"),
            -7 => format!("a1"),
            -8 => format!("a2"),
            -9 => format!("a3"),
            -10 => format!("a4"),
            -11 => format!("a5"),
            -12 => format!("a6"),
            -13 => format!("a7"),
            _ => format!("r{}", reg),
        }
    }

    pub fn to_string(&self) -> String {
        match self.tt {
            ADDI | ANDI | ORI | XORI | SLLI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
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
                    RiscvInstruction::reg_to_string(self.dest),
                    RiscvInstruction::reg_to_string(self.src1),
                    self.immediate
                )
            }
            LUI | AUIPC => format!(
                "\t{}\t{}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
            ),
            ADD | AND | OR | XOR | SLL | SUB => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SLT | DIV | REM | MUL | SRL => {
                let mut opcode = format!("\t{}", self.tt.to_string());
                if self.is_unsigned {
                    opcode += &"u";
                }
                format!(
                    "{}\t{}, {}, {}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.dest),
                    RiscvInstruction::reg_to_string(self.src1),
                    RiscvInstruction::reg_to_string(self.src2),
                )
            }
            J => format!("\t{}\t%L_{}_{}\n", self.tt.to_string(), self.label_function, self.label),
            JALR => {
                if self.src1 == 0 {
                    format!(
                        "\t{}\t{}, @{}\n",
                        self.tt.to_string(),
                        RiscvInstruction::reg_to_string(self.dest),
                        self.name
                    )
                } else {
                    format!(
                        "\t{}\t{}, {}\n",
                        self.tt.to_string(),
                        RiscvInstruction::reg_to_string(self.dest),
                        RiscvInstruction::reg_to_string(self.src1),
                    )
                }
            }
            BEQ | BNE => format!(
                "\t{}\t{}, {}, %L_{}_{}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2),
                self.label_function,
                self.label
            ),
            BLT | BGE => {
                let mut opcode = format!("\t{}", self.tt.to_string());
                if !self.is_unsigned {
                    opcode += &"u";
                }
                format!(
                    "{}\t{}, {}, %L_{}_{}\n",
                    opcode,
                    RiscvInstruction::reg_to_string(self.src1),
                    RiscvInstruction::reg_to_string(self.src2),
                    self.label_function,
                    self.label
                )
            }
            LH | LW | LB => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1)
            ),
            SB | SH | SW => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src2),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1),
            ),
            LABEL => format!("%L_{}_{}:\n", self.label_function, self.label),
            NOP => format!("\t{}\n", self.tt.to_string()),
            LABELFUNCTION => format!("\n@{}:\n", self.name),
        }
    }
}
