use url::{Url};

use forum::device::{DeviceQuery};
use version::{Version};

/// UUID Offset From Within A UDN
const UDN_UUID_OFFSET: usize = 5;

/// Exposes information available to typed queries.
pub trait TypedQuery {
    fn version(&self) -> Version;
}

/// Query containing no type information about what device it is querying.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GenericQuery {
    url: Url,
    udn: Vec<u8>
}

impl GenericQuery {
    pub fn new(url: Url, udn: Vec<u8>) -> GenericQuery {
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
pub enum QueryType {
    Root(GenericQuery),
    UUID(GenericQuery),
    Device(DeviceQuery),
    //Service(ServiceQuery)
}

impl QueryType {
  //  /// Creates a new QueryType from the given query and target.
//pub fn new<'a>(query: GenericQuery<'a>, target: TargetType) -> QueryType {
        
  //  }
  
  pub fn uuid(&self) -> &[u8] {
    match *self {
        QueryType::Root(ref n)    => n.uuid(),
        QueryType::UUID(ref n)    => n.uuid(),
        QueryType::Device(ref n)  => n.uuid()
        //QueryType::Service(ref n) => n.uuid()
    }
  }
}