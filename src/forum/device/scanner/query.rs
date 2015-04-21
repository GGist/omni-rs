use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ScannerQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl ScannerQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> ScannerQuery {
        ScannerQuery{ query: query, dev_type: dev_type }
    }
}