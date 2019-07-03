use api::BDAddr;

#[derive(Debug, Copy, Clone)]
pub enum CentralEvent {
    DeviceDiscovered(BDAddr),
    DeviceLost(BDAddr),
    DeviceUpdated(BDAddr),
    DeviceConnected(BDAddr),
    DeviceDisconnected(BDAddr),
}
