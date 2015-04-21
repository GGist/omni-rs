use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaRendererQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl MediaRendererQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> MediaRendererQuery {
        MediaRendererQuery{ query: query, dev_type: dev_type }
    }
}