use ::Result;

mod address_type;
mod bdaddr;
mod value_notification;
mod uuid;
mod characteristic;
mod char_prop_flags;
mod peripheral_properties;
mod peripheral;
mod central_event;
mod central;

pub use self::address_type::AddressType;
pub use self::bdaddr::BDAddr;
pub use self::value_notification::ValueNotification;
pub use self::uuid::UUID;
pub use self::characteristic::Characteristic;
pub use self::char_prop_flags::CharPropFlags;
pub use self::peripheral_properties::PeripheralProperties;
pub use self::peripheral::Peripheral;
pub use self::central_event::CentralEvent;
pub use self::central::Central;

pub type Callback<T> = Box<Fn(Result<T>) + Send>;
pub type CommandCallback = Callback<()>;
pub type RequestCallback = Callback<Vec<u8>>;
pub type NotificationHandler = Box<Fn(ValueNotification) + Send>;
pub type EventHandler = Box<Fn(CentralEvent) + Send>;
