#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AddressType {
    Random,
    Public,
}

impl Default for AddressType {
    fn default() -> Self { AddressType::Public }
}

impl AddressType {
    pub fn from_u8(v: u8) -> Option<AddressType> {
        match v {
            0 => Some(AddressType::Public),
            1 => Some(AddressType::Random),
            _ => None,
        }
    }

    pub fn num(&self) -> u8 {
        match *self {
            AddressType::Public => 0,
            AddressType::Random => 1
        }
    }
}
