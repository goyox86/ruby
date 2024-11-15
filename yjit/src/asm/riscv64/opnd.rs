/// This operand represents a register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RV64Reg {
    // Register index number
    pub reg_no: u8,
}

impl RV64Reg {
    // Register index number
    pub const fn new(reg_no: u8) -> RV64Reg {
        RV64Reg { reg_no }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RV64Mem {
    /// Base register number
    pub base_reg_no: u8,

    /// Constant offset from the base register
    pub offset: i16,
}

#[derive(Clone, Copy, Debug)]
pub enum RV64Opnd {
    // Dummy operand
    None,

    // Immediate value
    Imm(i64),

    // Unsigned immediate
    UImm(u64),

    // Register
    Reg(RV64Reg),

    // Memory
    Mem(RV64Mem),
}

impl RV64Opnd {
    /// Create a new immediate value operand.
    pub fn new_imm(value: i64) -> Self {
        RV64Opnd::Imm(value)
    }

    /// Create a new unsigned immediate value operand.
    pub fn new_uimm(value: u64) -> Self {
        RV64Opnd::UImm(value)
    }

    /// Creates a new register operand.
    pub const fn new_reg(reg: RV64Reg) -> Self {
        RV64Opnd::Reg(reg)
    }

    pub fn new_mem(base_reg_no: u8, offset: i16) -> Self {
        RV64Opnd::Mem(RV64Mem {
            base_reg_no,
            offset,
        })
    }

    /// Convenience function to check if this operand is a register.
    pub fn is_reg(&self) -> bool {
        match self {
            RV64Opnd::Reg(_) => true,
            _ => false,
        }
    }

    /// Unwrap a register from an operand.
    pub fn unwrap_reg(&self) -> RV64Reg {
        match self {
            RV64Opnd::Reg(reg) => *reg,
            _ => panic!("Expected register operand"),
        }
    }
}

// RISCV64 registers.
pub const X0_REG: RV64Reg = RV64Reg { reg_no: 0 };
pub const X1_REG: RV64Reg = RV64Reg { reg_no: 1 };
pub const X2_REG: RV64Reg = RV64Reg { reg_no: 2 };
pub const X3_REG: RV64Reg = RV64Reg { reg_no: 3 };
pub const X4_REG: RV64Reg = RV64Reg { reg_no: 4 };
pub const X5_REG: RV64Reg = RV64Reg { reg_no: 5 };
pub const X6_REG: RV64Reg = RV64Reg { reg_no: 6 };
pub const X7_REG: RV64Reg = RV64Reg { reg_no: 7 };
pub const X8_REG: RV64Reg = RV64Reg { reg_no: 8 };
pub const X9_REG: RV64Reg = RV64Reg { reg_no: 9 };
pub const X10_REG: RV64Reg = RV64Reg { reg_no: 10 };
pub const X11_REG: RV64Reg = RV64Reg { reg_no: 11 };
pub const X12_REG: RV64Reg = RV64Reg { reg_no: 12 };
pub const X13_REG: RV64Reg = RV64Reg { reg_no: 13 };
pub const X14_REG: RV64Reg = RV64Reg { reg_no: 14 };
pub const X15_REG: RV64Reg = RV64Reg { reg_no: 15 };
pub const X16_REG: RV64Reg = RV64Reg { reg_no: 16 };
pub const X17_REG: RV64Reg = RV64Reg { reg_no: 17 };
pub const X18_REG: RV64Reg = RV64Reg { reg_no: 18 };
pub const X19_REG: RV64Reg = RV64Reg { reg_no: 19 };
pub const X20_REG: RV64Reg = RV64Reg { reg_no: 20 };
pub const X21_REG: RV64Reg = RV64Reg { reg_no: 21 };
pub const X22_REG: RV64Reg = RV64Reg { reg_no: 22 };
pub const X23_REG: RV64Reg = RV64Reg { reg_no: 23 };
pub const X24_REG: RV64Reg = RV64Reg { reg_no: 24 };
pub const X25_REG: RV64Reg = RV64Reg { reg_no: 25 };
pub const X26_REG: RV64Reg = RV64Reg { reg_no: 26 };
pub const X27_REG: RV64Reg = RV64Reg { reg_no: 27 };
pub const X28_REG: RV64Reg = RV64Reg { reg_no: 28 };
pub const X29_REG: RV64Reg = RV64Reg { reg_no: 29 };
pub const X30_REG: RV64Reg = RV64Reg { reg_no: 30 };
pub const X31_REG: RV64Reg = RV64Reg { reg_no: 31 };

// RISCV64 Register Operands, ABI names.
// Hardwired to zero.
pub const ZERO: RV64Opnd = RV64Opnd::Reg(X0_REG);
// Return address.
pub const RA: RV64Opnd = RV64Opnd::Reg(X1_REG);
// Stack pointer.
pub const SP: RV64Opnd = RV64Opnd::Reg(X2_REG);
// Global pointer.
pub const GP: RV64Opnd = RV64Opnd::Reg(X3_REG);
// Thread pointer.
pub const TP: RV64Opnd = RV64Opnd::Reg(X4_REG);
// Frame pointer (FP is a saved register, S0).
pub const FP: RV64Opnd = S0;
// Temporaries.
pub const T0: RV64Opnd = RV64Opnd::Reg(X5_REG);
pub const T1: RV64Opnd = RV64Opnd::Reg(X6_REG);
pub const T2: RV64Opnd = RV64Opnd::Reg(X7_REG);
pub const T3: RV64Opnd = RV64Opnd::Reg(X28_REG);
pub const T4: RV64Opnd = RV64Opnd::Reg(X29_REG);
pub const T5: RV64Opnd = RV64Opnd::Reg(X30_REG);
pub const T6: RV64Opnd = RV64Opnd::Reg(X31_REG);
// Saved registers.
pub const S0: RV64Opnd = RV64Opnd::Reg(X8_REG);
pub const S1: RV64Opnd = RV64Opnd::Reg(X9_REG);
pub const S2: RV64Opnd = RV64Opnd::Reg(X18_REG);
pub const S3: RV64Opnd = RV64Opnd::Reg(X19_REG);
pub const S4: RV64Opnd = RV64Opnd::Reg(X20_REG);
pub const S5: RV64Opnd = RV64Opnd::Reg(X21_REG);
pub const S6: RV64Opnd = RV64Opnd::Reg(X22_REG);
pub const S7: RV64Opnd = RV64Opnd::Reg(X23_REG);
pub const S8: RV64Opnd = RV64Opnd::Reg(X24_REG);
pub const S9: RV64Opnd = RV64Opnd::Reg(X25_REG);
pub const S10: RV64Opnd = RV64Opnd::Reg(X26_REG);
pub const S11: RV64Opnd = RV64Opnd::Reg(X27_REG);
// Function argument/return values
pub const A0: RV64Opnd = RV64Opnd::Reg(X10_REG);
pub const A1: RV64Opnd = RV64Opnd::Reg(X11_REG);
// Function arguments
pub const A2: RV64Opnd = RV64Opnd::Reg(X12_REG);
pub const A3: RV64Opnd = RV64Opnd::Reg(X13_REG);
pub const A4: RV64Opnd = RV64Opnd::Reg(X14_REG);
pub const A5: RV64Opnd = RV64Opnd::Reg(X15_REG);
pub const A6: RV64Opnd = RV64Opnd::Reg(X16_REG);
pub const A7: RV64Opnd = RV64Opnd::Reg(X17_REG);

// C argument registers
pub const C_ARG_REGS: [RV64Opnd; 8] = [A0, A1, A2, A3, A4, A5, A6, A7];
pub const C_ARG_REGREGS: [RV64Reg; 8] = [
    X10_REG, X11_REG, X12_REG, X13_REG, X14_REG, X15_REG, X16_REG, X17_REG,
];
