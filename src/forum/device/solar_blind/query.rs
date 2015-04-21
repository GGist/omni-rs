use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SolarBlindQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl SolarBlindQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> SolarBlindQuery {
        SolarBlindQuery{ query: query, dev_type: dev_type }
    }
}