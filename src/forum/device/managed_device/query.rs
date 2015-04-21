use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ManagedDeviceQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl ManagedDeviceQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> ManagedDeviceQuery {
        ManagedDeviceQuery{ query: query, dev_type: dev_type }
    }
}