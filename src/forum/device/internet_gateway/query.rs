use forum::{GenericQuery};
use forum::device::{DeviceType};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InternetGatewayQuery {
    query:    GenericQuery,
    dev_type: DeviceType
}

impl InternetGatewayQuery {
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> InternetGatewayQuery {
        InternetGatewayQuery{ query: query, dev_type: dev_type }
    }
}