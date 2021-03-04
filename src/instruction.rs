use num_derive::FromPrimitive;
use strum::EnumString;

#[repr(u64)]
#[derive(Clone, Copy, Debug, EnumString, FromPrimitive, derive_more::Display)]
pub enum Instruction {
    LOAD,
    DLOAD,
    STORE,
    ADD,
    SUB,
    MULT,
    DIV,
    JUMP,
    JGE,
    JGT,
    JLE,
    JLT,
    JEQ,
    JNE,
    END,

    BP,
    NOOP,
    INT,
}

impl Instruction {
    pub fn takes_argument(self) -> bool {
        use Instruction::*;

        match self {
            END | BP | NOOP => false,
            _ => true
        }
    }
}

impl Instruction {
    pub fn takes_value(self) -> bool {
        match self {
            Self::END | Self::NOOP => false,
            _ => true
        }
    }
}
