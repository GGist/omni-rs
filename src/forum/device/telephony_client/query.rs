use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TelephonyClientQuery<'a> {
    query:    GenericQuery<'a>,
    dev_type: DeviceType
}

impl<'a> TelephonyClientQuery<'a> {
    pub fn new(query: GenericQuery<'a>, dev_type: DeviceType) -> TelephonyClientQuery<'a> {
        TelephonyClientQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl<'a> TypedQuery for TelephonyClientQuery<'a> {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}