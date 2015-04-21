use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SecurityCameraQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl SecurityCameraQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> SecurityCameraQuery {
        SecurityCameraQuery{ query: query, dev_type: dev_type }
    }
}