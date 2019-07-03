use std::fmt;
use std::fmt::{Display, Formatter, Debug};

use api::UUID::B16;
use api::UUID::B128;

/// A Bluetooth UUID. These can either be 2 bytes or 16 bytes long. UUIDs uniquely identify various
/// objects in the Bluetooth universe.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UUID {
    B16(u16),
    B128([u8; 16]),
}

impl UUID {
    pub fn size(&self) -> usize {
        match *self {
            B16(_) => 2,
            B128(_) => 16,
        }
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            B16(u) => write!(f, "{:02X}:{:02X}", u >> 8, u & 0xFF),
            B128(a) => {
                for i in (1..a.len()).rev() {
                    write!(f, "{:02X}:", a[i])?;
                }
                write!(f, "{:02X}", a[0])
            }
        }
    }
}

impl Debug for UUID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (self as &Display).fmt(f)
    }
}
