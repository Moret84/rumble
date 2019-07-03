use api::BDAddr;
use api::AddressType;

/// The properties of this peripheral, as determined by the advertising reports we've received for
/// it.
#[derive(Debug, Default, Clone)]
pub struct PeripheralProperties {
    /// The address of this peripheral
    pub address: BDAddr,
    /// The type of address (either random or public)
    pub address_type: AddressType,
    /// The local name. This is generally a human-readable string that identifies the type of device.
    pub local_name: Option<String>,
    /// The transmission power level for the device
    pub tx_power_level: Option<i8>,
    /// Unstructured data set by the device manufacturer
    pub manufacturer_data: Option<Vec<u8>>,
    /// Number of times we've seen advertising reports for this device
    pub discovery_count: u32,
    /// True if we've discovered the device before
    pub has_scan_response: bool,
}
