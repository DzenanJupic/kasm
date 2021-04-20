#![allow(non_snake_case)]

pub use error::Error;

pub type URS = usize;

pub type RAM<IRS> = Vec<(URS, IRS)>;

pub const DATA_REGISTERS: usize = 16;

pub mod cpu;
pub mod error;
pub mod instruction;
pub mod interrupt;
pub mod lexer;
