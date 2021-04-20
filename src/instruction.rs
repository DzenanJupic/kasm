use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::EnumString;

#[repr(usize)]
#[derive(Clone, Copy, Debug, EnumString, FromPrimitive, derive_more::Display, strum::EnumVariantNames)]
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
            Self::END | Self::BP | Self::NOOP => false,
            _ => true
        }
    }
}

impl TryFrom<i64> for Instruction {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_i64(value).ok_or(())
    }
}

impl TryFrom<i128> for Instruction {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Self::from_i128(value).ok_or(())
    }
}

impl TryFrom<f64> for Instruction {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::from_f64(value).ok_or(())
    }
}
