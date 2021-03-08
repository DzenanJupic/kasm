use thiserror::Error;

use crate::{IRS, URS};
use crate::lexer::code_token::CodeToken;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
    "The instruction code `{inst}` at BZ={BZ} is not a valid instruction\n\
    Note: Execute `/instructions` in the shell to get a list of all instructions"
    )]
    InvalidInstruction { inst: URS, BZ: URS },
    #[error(
    "The interrupt code `{int}` at BZ={BZ} is not a valid interrupt\n\
    Note: Execute `/interrupts` in the shell to get a list of all interrupts"
    )]
    InvalidInterrupt { int: IRS, BZ: URS },
    #[error("Attempted to divide {lhs} by zero at BZ={BZ}")]
    DivideByZero { lhs: IRS, BZ: URS },
    #[error(
    "Attempted to access Rx[{i}] at BZ={BZ}\n\
    Note: Rx has len {len}, and indexing starts at 0\n\
    Note: Indexes must be positive"
    )]
    InvalidRxIndex { i: IRS, len: usize, BZ: URS },
    #[error(
    "There are no more instructions at BZ={BZ}\n\
    Note: Always end your program with an `END` instruction"
    )]
    NoMoreInstructions { BZ: URS },

    #[error(
    "The CPU made {0} steps in a row without a break\n\
     Note: You probably entered an endless loop"
    )]
    TooManySteps(u64),

    #[error("The jump point {name} in line {line} is not defined in the document")]
    UndefinedJumpPoint { name: String, line: usize },
    #[error(
    "Failed to parse `{s}` in line {line}\n\
    Details: {err}"
    )]
    ParsingFailed { s: String, line: usize, err: ParseError },
    #[error(
    "The provided combination of tokens in line {line} is invalid\n\
    Details: {err}"
    )]
    InvalidTokenArrangement { line: usize, err: ParseError },

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse the unknown token `{token}`")]
    UnknownToken { token: String },
    #[error("The line `{line}` contains more then two tokens")]
    TooManyTokens { line: String },
    #[error("The token `{token}` may not be the first in a line")]
    TokenMayNotBeFirst { token: CodeToken },
    #[error("The token `{token}` may not be the second in a line")]
    TokenMayNotBeSecond { token: CodeToken },
    #[error("The token `{token}` does take an argument, but no argument was supplied")]
    TokenDoesTakeAnArgument { token: CodeToken },
    #[error("The token `{token}` does not take an argument")]
    TokenDoesNotTakeAnArgument { token: CodeToken },
}
