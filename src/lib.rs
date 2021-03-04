#![feature(str_split_once, bool_to_option, never_type)]
#![allow(non_snake_case)]

pub use error::Error;

pub type URS = u64;
pub type IRS = i64;
pub type RAM = Vec<(URS, IRS)>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub const DATA_REGISTERS: usize = 100;

pub mod cpu;
pub mod error;
pub mod instruction;
pub mod interrupt;
pub mod lexer;
