use std::fmt::{Debug, Display};
use std::str::FromStr;

use num_traits::{AsPrimitive, FromPrimitive};

use crate::URS;
use crate::error::ParseError;
use crate::instruction::Instruction;
use crate::lexer::jump_point::JumpPoint;

#[derive(Clone, Debug, derive_more::Display)]
pub enum CodeToken<IRS> {
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

impl<IRS: Debug + Display + FromStr + AsPrimitive<URS> + Copy + 'static> CodeToken<IRS>
    where URS: AsPrimitive<IRS> {
    pub fn from_str(s: &str) -> Result<Self, ParseError<IRS>> {
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
            Inst(inst) => inst as u64 as URS,
            Val(val) => val.as_(),
            Code(code) => code,
            JumpPoint(_) | JumpPointDeclaration(_) => {
                panic!("JumpPoints and JumpPointDeclarations must be resolved before converting to URS")
            }
        }
    }

    pub fn as_irs(&self) -> IRS {
        use CodeToken::*;

        match *self {
            Inst(inst) => (inst as URS).as_(),
            Val(val) => val as IRS,
            Code(code) => code.as_(),
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
                Instruction::from_usize(*c)
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

impl<IRS> FromStr for CodeToken<IRS>
    where
        IRS: Debug + Copy + Display + FromStr + AsPrimitive<URS> + 'static,
        usize: AsPrimitive<IRS> {
    type Err = ParseError<IRS>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CodeToken::from_str(s)
    }
}
