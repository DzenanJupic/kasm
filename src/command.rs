use num_derive::FromPrimitive;
use strum::{Display, EnumString, EnumVariantNames};

#[repr(u16)]
#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq, FromPrimitive, EnumVariantNames)]
pub enum Command {
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

    NOOP,
    INT,
}

impl Command {
    pub fn takes_value(self) -> bool {
        match self {
            Self::END | Self::NOOP => false,
            _ => true
        }
    }
}
