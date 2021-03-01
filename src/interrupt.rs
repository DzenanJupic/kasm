use num_derive::FromPrimitive;
use strum::{Display, EnumVariantNames};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, FromPrimitive, EnumVariantNames)]
pub enum Interrupt {
    Print,
    PrintBytes,
    DumpRegisters,
    StoreRam,
    StoreBZ,
    Step,
    Exec,
    ClearRam,
    Clear,
}
