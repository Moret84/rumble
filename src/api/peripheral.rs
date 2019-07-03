use ::Result;
use std::fmt::Debug;
use std::collections::BTreeSet;

use api::BDAddr;
use api::UUID;
use api::PeripheralProperties;
use api::Characteristic;
use api::CommandCallback;
use api::RequestCallback;
use api::NotificationHandler;

/// Peripheral is the device that you would like to communicate with (the "server" of BLE). This
/// struct contains both the current state of the device (its properties, characteristics, etc.)
/// as well as functions for communication.
pub trait Peripheral: Send + Sync + Clone + Debug {
    /// Returns the address of the peripheral.
    fn address(&self) -> BDAddr;

    /// Returns the set of properties associated with the peripheral. These may be updated over time
    /// as additional advertising reports are received.
    fn properties(&self) -> PeripheralProperties;

    /// The set of characteristics we've discovered for this device. This will be empty until
    /// `discover_characteristics` or `discover_characteristics_in_range` is called.
    fn characteristics(&self) -> BTreeSet<Characteristic>;

    /// Returns true iff we are currently connected to the device.
    fn is_connected(&self) -> bool;

    /// Creates a connection to the device. This is a synchronous operation; if this method returns
    /// Ok there has been successful connection. Note that peripherals allow only one connection at
    /// a time. Operations that attempt to communicate with a device will fail until it is connected.
    fn connect(&self) -> Result<()>;

    /// Terminates a connection to the device. This is a synchronous operation.
    fn disconnect(&self) -> Result<()>;

    /// Discovers all characteristics for the device. This is a synchronous operation.
    fn discover_characteristics(&self) -> Result<Vec<Characteristic>>;

    /// Discovers characteristics within the specified range of handles. This is a synchronous
    /// operation.
    fn discover_characteristics_in_range(&self, start: u16, end: u16) -> Result<Vec<Characteristic>>;

    /// Sends a command (`write-without-response`) to the characteristic. Takes an optional callback
    /// that will be notified in case of error or when the command has been successfully acked by the
    /// device.
    fn command_async(&self, characteristic: &Characteristic, data: &[u8], handler: Option<CommandCallback>);

    /// Sends a command (write without response) to the characteristic. Synchronously returns a
    /// `Result` with an error set if the command was not accepted by the device.
    fn command(&self, characteristic: &Characteristic, data: &[u8]) -> Result<()>;

    /// Sends a request (write) to the device. Takes an optional callback with either an error if
    /// the request was not accepted or the response from the device.
    fn request_async(&self, characteristic: &Characteristic,
                     data: &[u8], handler: Option<RequestCallback>);

    /// Sends a request (write) to the device. Synchronously returns either an error if the request
    /// was not accepted or the response from the device.
    fn request(&self, characteristic: &Characteristic,
               data: &[u8]) -> Result<Vec<u8>>;

    /// Sends a request (read) to the device. Takes an optional callback with either an error if
    /// the request was not accepted or the response from the device.
    fn read_async(&self, characteristic: &Characteristic, handler: Option<RequestCallback>);

    /// Sends a request (read) to the device. Synchronously returns either an error if the request
    /// was not accepted or the response from the device.
    fn read(&self, characteristic: &Characteristic) -> Result<Vec<u8>>;

    /// Sends a read-by-type request to device for the range of handles covered by the
    /// characteristic and for the specified declaration UUID. See
    /// [here](https://www.bluetooth.com/specifications/gatt/declarations) for valid UUIDs.
    /// Takes an optional callback that will be called with an error or the device response.
    fn read_by_type_async(&self, characteristic: &Characteristic,
                          uuid: UUID, handler: Option<RequestCallback>);

    /// Sends a read-by-type request to device for the range of handles covered by the
    /// characteristic and for the specified declaration UUID. See
    /// [here](https://www.bluetooth.com/specifications/gatt/declarations) for valid UUIDs.
    /// Synchronously returns either an error or the device response.
    fn read_by_type(&self, characteristic: &Characteristic,
                    uuid: UUID) -> Result<Vec<u8>>;

    /// Enables either notify or indicate (depending on support) for the specified characteristic.
    /// This is a synchronous call.
    fn subscribe(&self, characteristic: &Characteristic) -> Result<()>;

    /// Disables either notify or indicate (depending on support) for the specified characteristic.
    /// This is a synchronous call.
    fn unsubscribe(&self, characteristic: &Characteristic) -> Result<()>;

    /// Registers a handler that will be called when value notification messages are received from
    /// the device. This method should only be used after a connection has been established. Note
    /// that the handler will be called in a common thread, so it should not block.
    fn on_notification(&self, handler: NotificationHandler);
}
