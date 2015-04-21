use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BinaryLightQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl BinaryLightQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> BinaryLightQuery {
        BinaryLightQuery{ query: query, dev_type: dev_type }
    }
}