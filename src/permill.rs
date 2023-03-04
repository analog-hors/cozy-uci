use std::str::FromStr;
use std::num::IntErrorKind;

use thiserror::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permill(u16);

#[derive(Debug, Error, Clone, Copy)]
#[error("permill value is out of range (0..=1000)")]
pub struct PermillRangeError;

macro_rules! impl_convert {
    ($($type:ty),*) => {$(
        impl TryFrom<$type> for Permill {
            type Error = PermillRangeError;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                match value {
                    0..=1000 => Ok(Self(value as u16)),
                    _ => Err(PermillRangeError),
                }
            }
        }

        impl From<Permill> for $type {
            fn from(value: Permill) -> Self {
                value.0 as Self
            }
        }
    )*};
}
impl_convert!(i16, u16, i32, u32, i64, u64, i128, u128);

#[derive(Debug, Error, Clone, Copy)]
pub enum PermillParseError {
    #[error("not a valid number")]
    InvalidNumber,
    #[error("permill value is out of range (0..=1000)")]
    OutOfRange
}

impl FromStr for Permill {
    type Err = PermillParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PermillParseError::*;
        use IntErrorKind::*;

        match s.parse() {
            Ok(n @ 0..=1000) => Ok(Self(n)),
            Ok(_) => Err(OutOfRange),
            Err(e) if *e.kind() == PosOverflow => Err(OutOfRange),
            Err(e) if *e.kind() == NegOverflow => Err(OutOfRange),
            Err(_) => Err(InvalidNumber)
        }
    }
}
