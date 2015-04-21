use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TelephonyServerQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl TelephonyServerQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> TelephonyServerQuery {
        TelephonyServerQuery{ query: query, dev_type: dev_type }
    }
}