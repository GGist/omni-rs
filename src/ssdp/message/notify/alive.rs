use std::error::{Error};
use std::fmt::{self, Debug, Display, Formatter};

use hyper::header::{Headers, Location, Server, CacheDirective, CacheControl, Header};
use time::{Duration, PreciseTime};
use url::{Url};

use {SSDPError, SSDPResult};
use forum::{GenericQuery, QueryType, TargetType};
use forum::device::{DeviceQuery};
//use forum::service::{ServiceQuery};
use ssdp::{FieldPair};
use ssdp::header::{SearchPort, SecureLocation, BootID, NT, USN, ConfigID};
use ssdp::message::{self, MessageExt};

/// Represents AliveMessage versions pertaining to different UPnP versions.
#[derive(Copy, Clone)]
pub enum AliveVersion<'a> {
    /// UPnP Version 1.0.
    V10,
    /// UPnP Version 1.1.
    V11(&'a AliveExtV11),
    /// UPnP Version 2.0.
    V20(&'a AliveExtV20)
}

/// Represents an announcement made by some UPnP enabled device.
#[derive(Clone)]
pub struct AliveMessage {
    headers: Headers,
    created: PreciseTime,
    version: AliveVersionImpl,
    max_age: Duration,
    query:   QueryType
}

impl AliveMessage {
    fn new(headers: Headers) -> SSDPResult<AliveMessage> {
        let version = try!(AliveVersionImpl::new(&headers));
    
        let max_age = Duration::seconds(try!(first_max_age(&headers)) as i64);
        
        let query = try!(generate_query(&headers));
        
        Ok(AliveMessage{ headers: headers, created: PreciseTime::now(),
            version: version, max_age: max_age, query: query })
    }
    
    /// Returns whether or not the cache control set by the sender has expired.
    pub fn is_expired(&self) -> bool {
        self.created.to(PreciseTime::now()) > self.max_age
    }
    
    /// Returns the max-age directive attached to the message.
    pub fn max_age(&self) -> Duration {
        self.max_age
    }
    
    /// Returns the UPnP version number for the message.
    ///
    /// Information pertinent to each version is included.
    pub fn version<'a>(&'a self) -> AliveVersion<'a> {
        match self.version {
            AliveVersionImpl::V10 => AliveVersion::V10,
            AliveVersionImpl::V11(ref n) => AliveVersion::V11(n),
            AliveVersionImpl::V20(ref n) => AliveVersion::V20(n)
        }
    }
    
    /// Returns the query object associated with this message.
    pub fn query(&self) -> &QueryType {
        &self.query
    }
}

impl Debug for AliveMessage {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        try!(f.write_str("AliveMessage {"));
        
        try!(f.write_str(" headers: "));
        try!(Debug::fmt(&self.headers, f));
        
        try!(f.write_str(", created: Can't Format"));
        
        try!(f.write_str(", version: "));
        try!(Debug::fmt(&self.version, f));
        
        try!(f.write_str(", max_age: "));
        try!(Debug::fmt(&self.max_age, f));
        
        try!(f.write_str(", query: "));
        try!(Debug::fmt(&self.query, f));
        
        f.write_str(" }")
    }
}

/// Returns the query type for the target header field.
fn generate_query(headers: &Headers) -> SSDPResult<QueryType> {
    let ref notify_field = try!(headers.get::<NT>().ok_or(
        SSDPError::MissingHeader(NT::header_name())
    )).0;
    
    let url = try!(location_as_url(&headers));
    let udn = try!(udn_as_bytes(&headers));
    let generic_query = GenericQuery::new(url, udn);
    
    match TargetType::new(&notify_field) {
        Ok(TargetType::Root) => Ok(QueryType::Root(generic_query)),
        Ok(TargetType::UUID) => Ok(QueryType::UUID(generic_query)),
        Ok(TargetType::Device(n)) => {
            let device_query = DeviceQuery::new(generic_query, n);
            Ok(QueryType::Device(device_query))
        },
        Ok(TargetType::Service(n)) => {
            panic!("Unimplemented ssdp::message::notify::alive::generate_query")
            //let service_query = ServiceQuery::new(generic_query, n);
            //Ok(QueryType::Service(service_query))
        },
        Err(e) => Err(SSDPError::Other(Box::new(e) as Box<Error>))
    }
}

/// Returns the uuid portion of the URN header as bytes.
fn udn_as_bytes(headers: &Headers) -> SSDPResult<Vec<u8>> {
    let ref uuid = try!(headers.get::<USN>().ok_or(
        SSDPError::MissingHeader(USN::header_name())
    )).0;
    
    // UDN Is Simply The uuid Value With The "uuid:" Portion Prepended.
    // When FieldPair::UUID Is Displayed, The "uuid:" Portion Is Written.
    match *uuid {
        FieldPair::UUID(_) => Ok(uuid.to_string().into_bytes()),
        _ => Err(SSDPError::InvalidHeader(
                    USN::header_name(), "UUID Not Found As First Value"
                ))
    }
}

/// Returns the location header field as a Url.
fn location_as_url(headers: &Headers) -> SSDPResult<Url> {
    let location = try!(headers.get::<Location>().ok_or(
        SSDPError::MissingHeader(Location::header_name())
    ));
    
    Url::parse(location).map_err(|e|
        SSDPError::InvalidHeader(
            Location::header_name(), "Could Not Parse Location As A Url"
        )
    )
}

/// Returns first max-age directive found in the cache-control header.
fn first_max_age(headers: &Headers) -> SSDPResult<u32> {
    let ref cache_control_list = try!(headers.get::<CacheControl>().ok_or(
        SSDPError::MissingHeader(CacheControl::header_name())
    )).0;
    
    // Return First Max-Age Directive Found, Ignore Duplicates
    for i in cache_control_list {
        if let &CacheDirective::MaxAge(n) = i {
            return Ok(n)
        }
    }
    
    Err(SSDPError::InvalidHeader(CacheControl::header_name(),
        "No Max-Age Found In CacheControl"))
}

impl MessageExt for AliveMessage {
    fn check_header(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.headers.get_raw(name)
    }
}

/// Alive message versions for different UPnP versions.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum AliveVersionImpl {
    V10,
    V11(AliveExtV11Impl),
    V20(AliveExtV20Impl)
}

impl AliveVersionImpl {
    /// Create a new AliveVersionImpl object.
    fn new(headers: &Headers) -> SSDPResult<AliveVersionImpl> {
        let server = try!(headers.get::<Server>().ok_or(
            SSDPError::MissingHeader(Server::header_name())
        ));
        
        if server.contains(message::UPNP_10_VERSION_NAME) {
            Ok(AliveVersionImpl::V10)
        } else if server.contains(message::UPNP_11_VERSION_NAME) {
            let alive_ext = try!(AliveExtV11Impl::new(headers));
            Ok(AliveVersionImpl::V11(alive_ext))
        } else if server.contains(message::UPNP_20_VERSION_NAME) {
            let alive_ext = try!(AliveExtV20Impl::new(headers));
            Ok(AliveVersionImpl::V20(alive_ext))
        } else {
            Err(SSDPError::InvalidHeader(Server::header_name(), 
                                         "Invalid UPnP Version In Server Header"))
        }
    }
}

/// An extension trait for alive messages conforming to the UPnP 1.1 standard.
pub trait AliveExtV11 {
    /// A unique identifier that specifies the boot instance of the root device
    /// corresponding to the advertised device or service.
    ///
    /// This value will be the same for all alive messages that are coming from
    /// a device that has (1) not left the network since it last sent this message
    /// and (2) has not undergone any changes that triggered an UpdateMessage.
    fn boot_id(&self) -> u32;
    
    /// A configuration number for a device is a number that denotes a combination
    /// of a device's description documents (including embedded devices/services).
    ///
    /// This feature is mostly used to verify that your service objects are
    /// still sending their actions to a valid endpoint and that a query of the
    /// device description page does not need to occur again.
    fn config_id(&self) -> u32;
    
    /// A device can choose to respond to search requests on a port other than
    /// 1900 only if that port is unavailable.
    fn search_port(&self) -> Option<u16>;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AliveExtV11Impl {
    boot_id:   u32,
    config_id: u32,
    port:      Option<u16>
}

impl AliveExtV11Impl {
    fn new(header: &Headers) -> SSDPResult<AliveExtV11Impl> {
        let boot_id = try!(header.get::<BootID>().ok_or(
            SSDPError::MissingHeader(BootID::header_name())
        )).0;
        
        let config_id = try!(header.get::<ConfigID>().ok_or(
            SSDPError::MissingHeader(ConfigID::header_name())
        )).0;
        
        let port = header.get::<SearchPort>().map(|n| n.0);
        
        Ok(AliveExtV11Impl{ boot_id: boot_id, config_id: config_id, port: port })
    }
}

impl AliveExtV11 for AliveExtV11Impl {
    fn boot_id(&self) -> u32 {
        self.boot_id
    }
    
    fn config_id(&self) -> u32 {
        self.config_id
    }
    
    fn search_port(&self) -> Option<u16> {
        self.port
    }
}

/// An extension trait for alive messages conforming to the UPnP 2.0 standard.
pub trait AliveExtV20: AliveExtV11 {
    /// A secure location provides a means of retrieving the device description
    /// document over https.
    ///
    /// This string can be either an absolute or relative url.
    fn secure_location(&self) -> Option<&str>;
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AliveExtV20Impl {
    parent:     AliveExtV11Impl,
    secure_loc: Option<String>
}

impl AliveExtV20Impl {
    fn new(header: &Headers) -> SSDPResult<AliveExtV20Impl> {
        let parent = try!(AliveExtV11Impl::new(header));
        
        let secure_loc = header.get::<SecureLocation>().map(|n| n.0.clone());
        
        Ok(AliveExtV20Impl{ parent: parent, secure_loc: secure_loc })
    }
}

impl AliveExtV11 for AliveExtV20Impl {
    fn boot_id(&self) -> u32 {
        self.parent.boot_id
    }
    
    fn config_id(&self) -> u32 {
        self.parent.config_id
    }
    
    fn search_port(&self) -> Option<u16> {
        self.parent.port
    }
}

impl AliveExtV20 for AliveExtV20Impl {
    fn secure_location(&self) -> Option<&str> {
        match self.secure_loc {
            Some(ref n) => Some(&n[..]),
            None        => None
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Headers};
    use time::{Duration};
    
    use forum::{QueryType};
    use super::{AliveMessage, AliveVersion};
    

}