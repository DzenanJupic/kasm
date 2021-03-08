use num_derive::FromPrimitive;

#[repr(u64)]
#[derive(Clone, Copy, Debug, FromPrimitive, strum::EnumVariantNames)]
pub enum Interrupt {
    Print,
    PrintBytes,

    DumpA,
    DumpBZ,
    DumpRx,
    DumpRam,
}
