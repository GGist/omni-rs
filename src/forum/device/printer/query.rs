use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrinterQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl PrinterQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> PrinterQuery {
        PrinterQuery{ query: query, dev_type: dev_type }
    }
}