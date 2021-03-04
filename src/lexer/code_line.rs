use crate::{IRS, URS};
use crate::error::ParseError;

use super::code_token::CodeToken;

#[derive(Debug)]
pub enum CodeLine {
    SingleToken(CodeToken),
    DoubleToken(CodeToken, CodeToken),
}

impl CodeLine {
    pub fn from_str(s: &str) -> Result<Option<Self>, ParseError> {
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
            Self::SingleToken(ct) => (ct.as_urs(), 0),
            Self::DoubleToken(ct0, ct1) => (ct0.as_urs(), ct1.as_irs())
        }
    }

    pub fn check(&self) -> Result<(), ParseError> {
        match self {
            CodeLine::SingleToken(ct) => Self::check_single(ct),
            CodeLine::DoubleToken(ct0, ct1) => Self::check_double(ct0, ct1)
        }
    }

    fn check_single(ct: &CodeToken) -> Result<(), ParseError> {
        if !ct.can_be_first() {
            Err(ParseError::TokenMayNotBeFirst { token: ct.clone() })
        } else if ct.takes_second() {
            Err(ParseError::TokenDoesTakeAnArgument { token: ct.clone() })
        } else {
            Ok(())
        }
    }

    fn check_double(ct0: &CodeToken, ct1: &CodeToken) -> Result<(), ParseError> {
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
