use std::str::FromStr;

use num_traits::FromPrimitive;

use crate::{IRS, URS};
use crate::error::ParseError;
use crate::instruction::Instruction;
use crate::lexer::jump_point::JumpPoint;

#[derive(Clone, Debug, derive_more::Display)]
pub enum CodeToken {
    #[display(fmt = "{}", _0)]
    Inst(Instruction),
    #[display(fmt = "{}", _0)]
    Val(IRS),
    #[display(fmt = "{}", _0)]
    Code(URS),
    #[display(fmt = "{}", _0)]
    JumpPoint(JumpPoint),
    #[display(fmt = "{}", _0)]
    JumpPointDeclaration(JumpPoint),
}

impl CodeToken {
    pub fn from_str(s: &str) -> crate::Result<Self, ParseError> {
        let ref s = s.to_uppercase();

        if let Some(inst) = Self::parse(s) {
            return Ok(Self::Inst(inst));
        }

        if let Some(code) = Self::parse(s) {
            return Ok(Self::Code(code));
        }

        if let Some(val) = Self::parse(s) {
            return Ok(Self::Val(val));
        }

        if let Some(jump_point) = Self::parse(s) {
            return Ok(Self::JumpPoint(jump_point));
        }

        if s.ends_with(':') {
            if let Some(s) = s.get(..s.len() - 1) {
                if let Some(jump_point) = Self::parse(s) {
                    return Ok(Self::JumpPointDeclaration(jump_point))
                }
            }
        }

        Err(ParseError::UnknownToken { token: s.to_owned() })
    }

    pub fn as_urs(&self) -> URS {
        use CodeToken::*;

        match *self {
            Inst(inst) => inst as URS,
            Val(val) => val as URS,
            Code(code) => code as URS,
            JumpPoint(_) | JumpPointDeclaration(_) => {
                panic!("JumpPoints and JumpPointDeclarations must be resolved before converting to URS")
            }
        }
    }

    pub fn as_irs(&self) -> IRS {
        use CodeToken::*;

        match *self {
            Inst(inst) => inst as URS as IRS,
            Val(val) => val as IRS,
            Code(code) => code as IRS,
            JumpPoint(_) | JumpPointDeclaration(_) => {
                panic!("JumpPoints and JumpPointDeclarations must be resolved before converting to URS")
            }
        }
    }

    pub fn can_be_first(&self) -> bool {
        match self {
            Self::Inst(_) | Self::Code(_) | Self::JumpPointDeclaration(_) => true,
            Self::Val(_) | Self::JumpPoint(_) => false
        }
    }

    pub fn can_be_second(&self) -> bool {
        match self {
            Self::Inst(_) | Self::JumpPointDeclaration(_) => false,
            Self::Code(_) | Self::Val(_) | Self::JumpPoint(_) => true
        }
    }

    pub fn takes_second(&self) -> bool {
        match self {
            Self::Inst(inst) => inst.takes_value(),
            Self::Code(c) => {
                Instruction::from_u64(*c)
                    .map(Instruction::takes_value)
                    .unwrap_or(true)
            }
            Self::Val(_) | Self::JumpPoint(_) | Self::JumpPointDeclaration(_) => false
        }
    }

    pub fn as_jump_point_declaration(&self) -> Option<&JumpPoint> {
        match self {
            Self::JumpPointDeclaration(jp) => Some(jp),
            _ => None
        }
    }

    fn parse<T: FromStr>(s: &str) -> Option<T> {
        T::from_str(s).ok()
    }
}

impl FromStr for CodeToken {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
