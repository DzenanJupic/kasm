#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
    "Failed to load the next instruction at BZ={BZ}, since there are no more instructions\n\
    Note: Remember to always end your program with an `END` instruction"
    )]
    NoMoreInstructions { BZ: u16 },
    #[error(
    "Tried to parse `{inst}` at BZ=`{BZ}` to an instruction, but it is not a valid instruction\n\
    Note: To see a list of all instructions, execute `/instructions` in the shell"
    )]
    InvalidInstruction { inst: u16, BZ: u16 },
    #[error(
    "Tried to parse `{int}` at BZ=`{BZ}` to an interrupt, but it is not a valid interrupt\n\
    Note: To see a list of all interrupts, execute `/interrupts` in the shell"
    )]
    InvalidInterrupt { int: u16, BZ: u16 },
    #[error("Attempted to divide by zero")]
    DivideByZero,
    #[error(
    "Tried to execute an `END` instruction inside an interrupt\n\
     Note: This is invalid\n\
     Note: Execute `END` directly instead"
    )]
    EndedInInterrupt,
}
