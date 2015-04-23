use forum::{GenericQuery, TypedQuery};
use forum::device::{DeviceType};
use version::{Version};

/// Query for a device that is unimplemented at the moment.
///
/// Different than GenericQuery in that a GenericDeviceQuery is partially typed,
/// meaning that the advertised device type will still be checked when querying.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericDeviceQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl GenericDeviceQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> GenericDeviceQuery {
        GenericDeviceQuery{ query: query, dev_type: dev_type }
    }
    
    pub fn uuid(&self) -> &[u8] {
        self.query.uuid()
    }
}

impl TypedQuery for GenericDeviceQuery {
    fn version(&self) -> Version {
        self.dev_type.version()
    }
}