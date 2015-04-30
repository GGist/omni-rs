use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

/// Query for a device that is unimplemented at the moment.
///
/// Different than GenericQuery in that a GenericDeviceQuery is partially typed,
/// meaning that the advertised device type will still be checked when querying.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericDeviceQuery<'a> {
    query:    GenericQuery<'a>,
    dev_type: DeviceType
}

impl<'a> GenericDeviceQuery<'a> {
    pub fn new(query: GenericQuery<'a>, dev_type: DeviceType) -> GenericDeviceQuery<'a> {
        GenericDeviceQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl<'a> TypedQuery for GenericDeviceQuery<'a> {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}