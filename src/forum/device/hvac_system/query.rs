use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HVACSystemQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl HVACSystemQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> HVACSystemQuery {
        HVACSystemQuery{ query: query, dev_type: dev_type }
    }
}