use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BasicDeviceQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl BasicDeviceQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> BasicDeviceQuery {
        BasicDeviceQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for BasicDeviceQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}