use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaRendererQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl MediaRendererQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> MediaRendererQuery {
        MediaRendererQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for MediaRendererQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}