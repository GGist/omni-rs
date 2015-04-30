use url::{Url};

use forum::{TargetType};
use forum::device::{DeviceQuery};
use version::{Version};

/// Offset Of UUID Inside A UDN
const UDN_UUID_OFFSET: usize = 5;

/// Exposes information available to typed queries.
pub trait TypedQuery {
    fn version(&self) -> Version;
}

/// Query containing no type information about what device it is querying.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericQuery<'a> {
    url: &'a Url,
    udn: &'a [u8]
}

impl<'a> GenericQuery<'a> {
    pub fn new(url: &'a Url, udn: &'a [u8]) -> GenericQuery<'a> {
        GenericQuery{ url: url, udn: udn }
    }
    
    pub fn uuid(&self) -> &[u8] {
        &self.udn[UDN_UUID_OFFSET..]
    }
    
    /*pub fn query(&self) -> Result<Device> {
        // TODO: Fill In
        Ok(()) 
    }*/
}

/// Represents typed and untyped query objects.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum QueryType<'a> {
    Root(GenericQuery<'a>),
    UUID(GenericQuery<'a>),
    Device(DeviceQuery<'a>),
    //Service(ServiceQuery)
}

impl<'a> QueryType<'a> {
    /// Creates a new QueryType from the given query and target.
    pub fn new(query: GenericQuery<'a>, target: TargetType) -> QueryType<'a> {
        match target {
            TargetType::Root => QueryType::Root(query),
            TargetType::UUID => QueryType::UUID(query),
            TargetType::Device(n)  => QueryType::Device(DeviceQuery::new(query, n)),
            TargetType::Service(n) => panic!("Unimplemented forum::query::QueryType::new")
        }
    }
  
    pub fn uuid(&self) -> &[u8] {
        match *self {
            QueryType::Root(ref n)    => n.uuid(),
            QueryType::UUID(ref n)    => n.uuid(),
            QueryType::Device(ref n)  => n.uuid()
            //QueryType::Service(ref n) => n.uuid()
        }
    }
}