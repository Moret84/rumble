use std::fmt;
use std::fmt::{Display, Formatter, Debug};

/// Stores the 6 byte address used to identify Bluetooth devices.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
#[repr(C)]
pub struct BDAddr {
    pub address: [ u8 ; 6usize ]
}

impl Display for BDAddr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a = self.address;
        write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
               a[5], a[4], a[3], a[2], a[1], a[0])
    }
}

impl Debug for BDAddr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (self as &Display).fmt(f)
    }
}
