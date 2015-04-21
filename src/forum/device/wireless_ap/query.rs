use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct WirelessAPQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl WirelessAPQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> WirelessAPQuery {
        WirelessAPQuery{ query: query, dev_type: dev_type }
    }
}