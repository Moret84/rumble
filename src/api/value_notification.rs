/// A notification sent from a peripheral due to a change in a value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueNotification {
    /// The handle that has changed.
    pub handle: u16,
    /// The new value of the handle.
    pub value: Vec<u8>,
}
