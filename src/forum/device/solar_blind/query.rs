use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SolarBlindQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl SolarBlindQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> SolarBlindQuery {
        SolarBlindQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for SolarBlindQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}