use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DimmableLightQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl DimmableLightQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> DimmableLightQuery {
        DimmableLightQuery{ query: query, dev_type: dev_type }
    }
}