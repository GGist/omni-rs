use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TelephonyClientQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl TelephonyClientQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> TelephonyClientQuery {
        TelephonyClientQuery{ query: query, dev_type: dev_type }
    }
}