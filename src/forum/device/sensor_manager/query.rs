use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SensorManagerQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl SensorManagerQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> SensorManagerQuery {
        SensorManagerQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for SensorManagerQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}