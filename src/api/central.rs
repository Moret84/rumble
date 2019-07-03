use ::Result;

use api::Peripheral;
use api::EventHandler;
use api::BDAddr;

/// Central is the "client" of BLE. It's able to scan for and establish connections to peripherals.
pub trait Central<P : Peripheral>: Send + Sync + Clone {
    /// Registers a function that will receive notifications when events occur for this Central
    /// module. See [`Event`](enum.CentralEvent.html) for the full set of events. Note that the
    /// handler will be called in a common thread, so it should not block.
    fn on_event(&self, handler: EventHandler);

    /// Starts a scan for BLE devices. This scan will generally continue until explicitly stopped,
    /// although this may depend on your bluetooth adapter. Discovered devices will be announced
    /// to subscribers of `on_event` and will be available via `peripherals()`.
    fn start_scan(&self) -> Result<()>;

    /// Control whether to use active or passive scan mode to find BLE devices. Active mode scan
    /// notifies advertises about the scan, whereas passive scan only receives data from the
    /// advertiser. Defaults to use active mode.
    fn active(&self, enabled: bool);

    /// Control whether to filter multiple advertisements by the same peer device. Receving
    // can be useful for some applications. E.g. when using scan to collect information from
    /// beacons that update data frequently. Defaults to filter duplicate advertisements.
    fn filter_duplicates(&self, enabled: bool);

    /// Stops scanning for BLE devices.
    fn stop_scan(&self) -> Result<()>;

    /// Returns the list of [`Peripherals`](trait.Peripheral.html) that have been discovered so far.
    /// Note that this list may contain peripherals that are no longer available.
    fn peripherals(&self) -> Vec<P>;

    /// Returns a particular [`Peripheral`](trait.Peripheral.html) by its address if it has been
    /// discovered.
    fn peripheral(&self, address: BDAddr) -> Option<P>;
}
