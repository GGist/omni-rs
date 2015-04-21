use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaServerQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl MediaServerQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> MediaServerQuery {
        MediaServerQuery{ query: query, dev_type: dev_type }
    }
}