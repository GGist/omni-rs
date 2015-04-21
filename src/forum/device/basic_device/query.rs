use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BasicDeviceQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl BasicDeviceQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> BasicDeviceQuery {
        BasicDeviceQuery{ query: query, dev_type: dev_type }
    }
}