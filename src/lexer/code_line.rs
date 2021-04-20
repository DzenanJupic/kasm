use std::fmt::{Debug, Display};
use std::str::FromStr;

use num_traits::{AsPrimitive, Num};

use crate::URS;
use crate::error::ParseError;

use super::code_token::CodeToken;

#[derive(Debug)]
pub enum CodeLine<IRS> {
    SingleToken(CodeToken<IRS>),
    DoubleToken(CodeToken<IRS>, CodeToken<IRS>),
}

impl<IRS: Num + Debug + Display + FromStr + AsPrimitive<URS> + Copy + 'static> CodeLine<IRS>
    where URS: AsPrimitive<IRS> {
    pub fn from_str(s: &str) -> Result<Option<Self>, ParseError<IRS>> {
        let code = trim_comments(s);
        let mut code_parts = code.split_ascii_whitespace();

        let ct0 = match code_parts.next() {
            Some(token) => CodeToken::from_str(token)?,
            None => return Ok(None)
        };

        let cl = match code_parts.next() {
            Some(token) => {
                let ct1 = CodeToken::from_str(token)?;
                Self::DoubleToken(ct0, ct1)
            },
            None => Self::SingleToken(ct0)
        };

        match code_parts.next() {
            Some(_) => Err(ParseError::TooManyTokens { line: s.to_owned() }),
            None => Ok(Some(cl))
        }
    }

    pub fn as_urs_irs(&self) -> (URS, IRS) {
        match self {
            Self::SingleToken(ct) => (ct.as_urs(), IRS::zero()),
            Self::DoubleToken(ct0, ct1) => (ct0.as_urs(), ct1.as_irs())
        }
    }

    pub fn check(&self) -> Result<(), ParseError<IRS>> {
        match self {
            CodeLine::SingleToken(ct) => Self::check_single(ct),
            CodeLine::DoubleToken(ct0, ct1) => Self::check_double(ct0, ct1)
        }
    }

    fn check_single(ct: &CodeToken<IRS>) -> Result<(), ParseError<IRS>> {
        if !ct.can_be_first() {
            Err(ParseError::TokenMayNotBeFirst { token: ct.clone() })
        } else if ct.takes_second() {
            Err(ParseError::TokenDoesTakeAnArgument { token: ct.clone() })
        } else {
            Ok(())
        }
    }

    fn check_double(ct0: &CodeToken<IRS>, ct1: &CodeToken<IRS>) -> Result<(), ParseError<IRS>> {
        if !ct0.can_be_first() {
            Err(ParseError::TokenMayNotBeFirst { token: ct0.clone() })
        } else if !ct1.can_be_second() {
            Err(ParseError::TokenMayNotBeSecond { token: ct1.clone() })
        } else if !ct0.takes_second() {
            Err(ParseError::TokenDoesNotTakeAnArgument { token: ct0.clone() })
        } else {
            Ok(())
        }
    }
}

fn trim_comments(s: &str) -> &str {
    match s.split_once(';') {
        Some((code, _)) => code,
        None => s
    }
}
