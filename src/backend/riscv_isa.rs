#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum RiscvInstructionType {
    ADDI,
    SLTI,
    ANDI,
    ORI,
    XORI,
    SLLI,
    SRLI,
    SRAI,
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
    BGT,
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
            SRAI => format!("srai"),
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
            BGT => format!("bgt"),
            LB => format!("lb"),
            LH => format!("lh"),
            LW => format!("lw"),
            SB => format!("sb"),
            SH => format!("sh"),
            SW => format!("sw"),
            DIV => format!("div"),
            REM => format!("rem"),
            MUL => format!("mul"),
            LABEL => format!(""),
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
            _ => format!("r{}", reg),
        }
    }

    pub fn to_string(&self) -> String {
        match self.tt {
            ADDI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            SLTI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            ANDI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            ORI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            XORI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            SLLI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            SRLI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            SRAI => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate
            ),
            LUI => format!("\t{} TBD", self.tt.to_string()),
            AUIPC => format!("\t{} TBD", self.tt.to_string()),
            ADD => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SLT => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            AND => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            OR => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            XOR => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SLL => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SRL => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SUB => format!(
                "\t{}\t{}, {}, {}\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                RiscvInstruction::reg_to_string(self.src1),
                RiscvInstruction::reg_to_string(self.src2)
            ),
            J => format!("{} TBD", self.tt.to_string()),
            JALR => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1)
            ),
            BEQ => format!("{} TBD", self.tt.to_string()),
            BNE => format!("{} TBD", self.tt.to_string()),
            BLT => format!("{} TBD", self.tt.to_string()),
            BGT => format!("{} TBD", self.tt.to_string()),
            LB => format!(
                "\t{}\t {}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1)
            ),
            LH => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1)
            ),
            LW => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.dest),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src1)
            ),
            SB => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SH => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src2)
            ),
            SW => format!(
                "\t{}\t{}, {}({})\n",
                self.tt.to_string(),
                RiscvInstruction::reg_to_string(self.src1),
                self.immediate,
                RiscvInstruction::reg_to_string(self.src2)
            ),
            DIV => format!("{} TBD", self.tt.to_string()),
            REM => format!("{} TBD", self.tt.to_string()),
            MUL => format!("{} TBD", self.tt.to_string()),
            LABEL => format!("L_{}_{}: TBD", self.label_function, self.label),
            NOP => format!("\t{}", self.tt.to_string()),
        }
    }
}
