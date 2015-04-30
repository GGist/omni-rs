use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct WirelessAPQuery<'a> {
    query:    GenericQuery<'a>,
    dev_type: DeviceType
}

impl<'a> WirelessAPQuery<'a> {
    pub fn new(query: GenericQuery<'a>, dev_type: DeviceType) -> WirelessAPQuery<'a> {
        WirelessAPQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl<'a> TypedQuery for WirelessAPQuery<'a> {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}