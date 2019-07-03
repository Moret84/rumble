use std::fmt;
use std::fmt::{Display, Formatter};

use api::CharPropFlags;
use api::UUID;

/// A Bluetooth characteristic. Characteristics are the main way you will interact with other
/// bluetooth devices. Characteristics are identified by a UUID which may be standardized
/// (like 0x2803, which identifies a characteristic for reading heart rate measurements) but more
/// often are specific to a particular device. The standard set of characteristics can be found
/// [here](https://www.bluetooth.com/specifications/gatt/characteristics).
///
/// A characteristic may be interacted with in various ways depending on its properties. You may be
/// able to write to it, read from it, set its notify or indicate status, or send a command to it.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Characteristic {
    /// The start of the handle range that contains this characteristic.
    pub start_handle: u16,
    /// The end of the handle range that contains this characteristic.
    pub end_handle: u16,
    /// The value handle of the characteristic.
    pub value_handle: u16,
    /// The UUID for this characteristic. This uniquely identifies its behavior.
    pub uuid: UUID,
    /// The set of properties for this characteristic, which indicate what functionality it
    /// supports. If you attempt an operation that is not supported by the characteristics (for
    /// example setting notify on one without the NOTIFY flag), that operation will fail.
    pub properties: CharPropFlags,
}

impl Display for Characteristic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "handle: 0x{:04X}, char properties: 0x{:02X}, \
                   char value handle: 0x{:04X}, end handle: 0x{:04X}, uuid: {:?}",
               self.start_handle, self.properties,
               self.value_handle, self.end_handle, self.uuid)
    }
}
