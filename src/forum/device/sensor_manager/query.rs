use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SensorManagerQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl SensorManagerQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> SensorManagerQuery {
        SensorManagerQuery{ query: query, dev_type: dev_type }
    }
}