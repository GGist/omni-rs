use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HVACSystemQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl HVACSystemQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> HVACSystemQuery {
        HVACSystemQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for HVACSystemQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}