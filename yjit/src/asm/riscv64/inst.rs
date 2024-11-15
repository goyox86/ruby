// 31        25 24     20 19     15 14  12 11      7 6            0
// +----------+---------+---------+------+---------+-------------+
// | funct7   | rs2     | rs1     |funct3| rd     | opcode      |
// +----------+---------+---------+------+---------+-------------+
//    7 bits    5 bits    5 bits   3 bits  5 bits    7 bits

use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    // RV32I/RV64I R-type
    Add = 0b0110011,   // Add
    Addw = 0b01110011, // Addw
    SUB,               // Subtract
    SLL,               // Shift Left Logical
    SLT,               // Set Less Than
    SLTU,              // Set Less Than Unsigned
    XOR,               // Exclusive OR
    SRL,               // Shift Right Logical
    SRA,               // Shift Right Arithmetic
    OR,                // OR
    AND,               // AND

    // RV64I R-type
    ADDW, // Add Word
    SUBW, // Subtract Word
    SLLW, // Shift Left Logical Word
    SRLW, // Shift Right Logical Word
    SRAW, // Shift Right Arithmetic Word

    // M Extension (Integer Multiplication and Division)
    MUL,    // Multiply
    MULH,   // Multiply High Signed x Signed
    MULHSU, // Multiply High Signed x Unsigned
    MULHU,  // Multiply High Unsigned x Unsigned
    DIV,    // Divide Signed
    DIVU,   // Divide Unsigned
    REM,    // Remainder Signed
    REMU,   // Remainder Unsigned

    // RV64M Extension
    MULW,  // Multiply Word
    DIVW,  // Divide Signed Word
    DIVUW, // Divide Unsigned Word
    REMW,  // Remainder Signed Word
    REMUW, // Remainder Unsigned Word
}

pub trait R {
    fn opcode(&self) -> Opcode;
    fn rd(&self) -> u8;
    fn func3(&self) -> u8;
    fn rs1(&self) -> u8;
    fn rs2(&self) -> u8;
    fn func7(&self) -> u8;
}

#[derive(Debug, Clone, Copy)]
struct Add {
    rd: u8,
    rs1: u8,
    rs2: u8,
}

impl R for Add {
    fn opcode(&self) -> Opcode {
        Opcode::Add
    }

    fn rd(&self) -> u8 {
        self.rd
    }

    fn func3(&self) -> u8 {
        0b000
    }

    fn rs1(&self) -> u8 {
        self.rs1
    }

    fn rs2(&self) -> u8 {
        self.rs2
    }

    fn func7(&self) -> u8 {
        0b0000000
    }
}

#[derive(Debug, Clone, Copy)]
struct Addw {
    rd: u8,
    rs1: u8,
    rs2: u8,
}

impl R for Addw {
    fn opcode(&self) -> Opcode {
        Opcode::Addw
    }

    fn rd(&self) -> u8 {
        self.rd
    }

    fn func3(&self) -> u8 {
        0b000
    }

    fn rs1(&self) -> u8 {
        self.rs1
    }

    fn rs2(&self) -> u8 {
        self.rs2
    }

    fn func7(&self) -> u8 {
        0b0000000
    }
}

impl From<Add> for u32 {
    fn from(inst: Add) -> u32 {
        RCodec::encode::<Add, u32>(inst)
    }
}

impl From<Addw> for u32 {
    fn from(inst: Addw) -> u32 {
        RCodec::encode::<Addw, u32>(inst)
    }
}

pub struct RCodec;

impl RCodec {
    pub fn encode<I: R, O: Into<std::ops::BitOr<u32, u32>>>(inst: I) -> O {
        0 | ((inst.func7().into()) << 25)
            | ((inst.rs2().into()) << 20)
            | ((inst.rs1().into()) << 15)
            | ((inst.func3().into()) << 12)
            | ((inst.rd().into()) << 7)
            | (inst.opcode().into())
    }
}
