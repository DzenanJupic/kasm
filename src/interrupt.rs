use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(usize)]
#[derive(Clone, Copy, Debug, FromPrimitive, strum::EnumVariantNames)]
pub enum Interrupt {
    Print,
    PrintBytes,

    DumpA,
    DumpBZ,
    DumpRx,
    DumpRam,
}

impl TryFrom<i64> for Interrupt {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_i64(value).ok_or(())
    }
}

impl TryFrom<i128> for Interrupt {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Self::from_i128(value).ok_or(())
    }
}

impl TryFrom<f64> for Interrupt {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::from_f64(value).ok_or(())
    }
}
