use std::error::{Error};
use std::fmt::{self, Debug, Display, Formatter};
use std::str;

use hyper::header::{Location, Server, CacheDirective, CacheControl, Header, Host, Headers};
use time::{Duration, PreciseTime};
use url::{Url};

use {SSDPError, SSDPResult};
use forum::{GenericQuery, QueryType, TargetType};
use forum::device::{DeviceQuery};
//use forum::service::{ServiceQuery};
use ssdp::{FieldPair};
use ssdp::header::{HeaderView, SearchPort, SecureLocation, BootID, NT, USN, ConfigID};
use ssdp::message::{self, MessageExt};

/// Represents ByeByeMessage versions pertaining to different UPnP versions.
///
/// Since ByeByeMessage versions are inferred, the version presented here may
/// be less than or equal to the actual UPnP version in use.
#[derive(Copy, Clone)]
pub enum ByeByeVersion<'a> {
    /// UPnP Version 1.0.
    V10,
    /// UPnP Version 1.1.
    V11(&'a ByeByeExtV11)
}

/// Represents a message signifying that a device is gracefully shutting down.
#[derive(Debug, Clone)]
pub struct ByeByeMessage {
    headers: Headers,
    target: TargetType
}

impl ByeByeMessage {
    /// Create a new ByeByeMessage from the given header.
    fn new(headers: Headers) -> SSDPResult<ByeByeMessage> {
        try!(notify::check_multicast_host(&headers));
        
        let target = try!(extract_target(headers));
        
        Ok(AliveMessage{ headers: headers, created: PreciseTime::now(),
            version: version, max_age: max_age, query: query })
    }
    
    
}

fn byebye_pieces<T>(headers: T) -> SSDPResult<()> where T: HeaderView {
    // Extract Required Headers
    let ref host_name = try!(try_view_header::<T, Host>(&headers)).hostname;
    let ref notify_type = try!(try_view_header::<T, NT>(&headers)).0;
    let &USN(ref usn_uuid, ref usn_type) = try!(try_view_header::<T, USN>(&headers));
    
    // Validate Portions Of Message
    try!(check_multicast_host(&host_name[..]));
    try!(check_nt_usn_rules(notify_type, usn_uuid, usn_type));
    
    // Create Alive Pieces
    let version = try!(AliveVersionImpl::new(&headers));
    let max_age = try!(first_max_age(&cache_control[..]));
    let url = try!(location_as_url(&location[..]));
    let target = try!(notify_as_target(notify_type));
    
    Ok((version, Duration::seconds(max_age as i64), url, target))
}

/// Returns first max-age directive found in the cache-control header.
fn first_max_age<T>(headers: T) -> SSDPResult<u32> where T: HeaderView {
    let ref cache_control_list = try!(headers.view::<CacheControl>().ok_or(
        SSDPError::MissingHeader(CacheControl::header_name())
    )).0;
    
    // Return First Max-Age Directive Found, Ignore Duplicates
    for i in cache_control_list {
        if let &CacheDirective::MaxAge(n) = i {
            return Ok(n)
        }
    }
    
    Err(SSDPError::InvalidHeader(CacheControl::header_name(),
        "No Max-Age Found"))
}

/// Returns the query type for the target header field.
fn generate_query<T>(headers: T) -> SSDPResult<QueryType> where T: HeaderView {
    let ref notify_field = try!(headers.view::<NT>().ok_or(
        SSDPError::MissingHeader(NT::header_name())
    ));
    let ref usn_field = try!(headers.view::<USN>().ok_or(
        SSDPError::MissingHeader(USN::header_name())
    ));

    let url = try!(location_as_url(&headers));
    let udn = try!(try_extract_udn(&notify_field, &usn_field));
    
    let generic_query = GenericQuery::new(url, udn);
    
    match TargetType::new(&notify_field.0) {
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

/// Returns the location header field as a Url.
fn location_as_url<T>(headers: T) -> SSDPResult<Url> where T: HeaderView {
    let location = try!(headers.view::<Location>().ok_or(
        SSDPError::MissingHeader(Location::header_name())
    ));
    
    Url::parse(location).map_err(|e|
        SSDPError::InvalidHeader(
            Location::header_name(), "Could Not Parse Location As A Url"
        )
    )
}

/// Try to extract a udn value from the given headers after first validating
/// them against UPnP NT and USN field matching rules.
fn try_extract_udn(notify_field: &NT, usn_field: &USN) -> SSDPResult<Vec<u8>> {
    let usn_uuid = match usn_field.0 {
        FieldPair::UUID(ref n) => &n[..],
        _ => return Err(SSDPError::InvalidHeader(USN::header_name(),
                        "UUID Not Found As First Field"))
    };

    // Process According To NT And USN Relationship Rules In The UPnP Standard.
    // We Are Only Comparing Bytes Here Because The NT Will Be Validated When
    // We Generate A TargetType, So Any Invalid NT Values Will Be Caught Later.
    match notify_field.0 {
        FieldPair::UUID(ref n) => {
            try!(compare_usn_uuid(&n[..], usn_uuid));
            // USN Header Should Have Empty Second Field When NT Is UUID
            if usn_field.1.is_some() {
                return Err(SSDPError::InvalidHeader(USN::header_name(),
                    "Second Field In USN Header Is Present"))
            }
        },
        FieldPair::UPnP(ref n) => try!(compare_usn_upnp(&n[..], &usn_field.1)),
        FieldPair::URN(ref n)  => try!(compare_usn_urn(&n[..], &usn_field.1)),
        FieldPair::Unknown(..) => return Err(SSDPError::InvalidHeader(NT::header_name(),
                                             "Unknown Key Is Not A Valid Key"))
    };
    
    // We Already Know That USN Is Of Type FieldPair::UUID
    Ok(usn_field.0.to_string().into_bytes())
}

/// Compare the uuid bytes of the NT and USN fields.
fn compare_usn_uuid(nt_bytes: &[u8], usn_bytes: &[u8]) -> SSDPResult<()> {
    if nt_bytes != usn_bytes {
        Err(SSDPError::InvalidHeader(USN::header_name(), 
            "First Field Has A uuid That Does Not Match NT"))
    } else { Ok(()) }
}

/// Compare the upnp bytes of the NT and USN fields.
fn compare_usn_upnp(nt_bytes: &[u8], usn_field: &Option<FieldPair>) -> SSDPResult<()> {
    match *usn_field {
        Some(FieldPair::UPnP(ref n)) => {
            if nt_bytes != &n[..] {
                Err(SSDPError::InvalidHeader(USN::header_name(),
                    "Second Field Has A Value That Does Not Match NT"))
            } else { Ok(()) }
        },
        _ => Err(SSDPError::InvalidHeader(USN::header_name(),
                 "Second Field Has A Key That Does Not Match NT"))
    }
}

/// Compare the urn bytes of the NT and USN fields.
fn compare_usn_urn(nt_bytes: &[u8], usn_field: &Option<FieldPair>) -> SSDPResult<()> {
    match *usn_field {
        Some(FieldPair::URN(ref n)) => {
            if nt_bytes != &n[..] {
                Err(SSDPError::InvalidHeader(USN::header_name(),
                    "Second Field Has A Value That Does Not Match NT"))
            } else { Ok(()) }
        },
        _ => Err(SSDPError::InvalidHeader(USN::header_name(),
                 "Second Field Has A Key That Does Not Match NT"))
    }
}

impl<T> MessageExt for AliveMessage<T> where T: HeaderView {
    fn check_header(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.headers.view_raw(name)
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
    fn new<T>(headers: T) -> SSDPResult<AliveVersionImpl> where T: HeaderView {
        let server = try!(headers.view::<Server>().ok_or(
            SSDPError::MissingHeader(Server::header_name())
        ));
        
        // TODO: Change this so that just the UPnP/ is matched and gather the version
        // number to convert to some enum like UPnPVersion::1.0, etc.
        if server.contains(message::UPNP_10_VERSION_NAME) {
            Ok(AliveVersionImpl::V10)
        } else if server.contains(message::UPNP_11_VERSION_NAME) {
            let alive_ext = try!(AliveExtV11Impl::new(&headers));
            Ok(AliveVersionImpl::V11(alive_ext))
        } else if server.contains(message::UPNP_20_VERSION_NAME) {
            let alive_ext = try!(AliveExtV20Impl::new(&headers));
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
struct AliveExtV11Impl {
    boot_id:   u32,
    config_id: u32,
    port:      Option<u16>
}

impl AliveExtV11Impl {
    fn new<T>(header: T) -> SSDPResult<AliveExtV11Impl> where T: HeaderView {
        let boot_id = try!(header.view::<BootID>().ok_or(
            SSDPError::MissingHeader(BootID::header_name())
        )).0;
        
        let config_id = try!(header.view::<ConfigID>().ok_or(
            SSDPError::MissingHeader(ConfigID::header_name())
        )).0;
        
        let port = header.view::<SearchPort>().map(|n| n.0);
        
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
struct AliveExtV20Impl {
    parent:     AliveExtV11Impl,
    secure_loc: Option<String>
}

impl AliveExtV20Impl {
    fn new<T>(header: T) -> SSDPResult<AliveExtV20Impl> where T: HeaderView {
        let parent = try!(AliveExtV11Impl::new(&header));
        
        let secure_loc = header.view::<SecureLocation>().map(|n| n.0.clone());
        
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
    use time::{Duration};
    use hyper::header::{Host, CacheControl, Location, Server};

    use forum::{QueryType};
    use ssdp::header::{HeaderView, SearchPort, SecureLocation, BootID, NT, USN,
                       ConfigID, NTS};
    use ssdp::header::mock::{MockHeaderView};
    use super::{AliveMessage, AliveVersion};
    
    #[test]
    fn positive_alive_message() {
        let mut headers = MockHeaderView::new();
        
        headers.insert::<Host>("239.255.255.250:1900");
        headers.insert::<CacheControl>("max-age=1800");
        headers.insert::<Location>("http://192.168.0.1/desc.xml");
        headers.insert::<NT>("upnp:rootdevice");
        headers.insert::<NTS>("ssdp:alive");
        headers.insert::<Server>("Windows/3.1 UPnP/1.0 omni/0.0.1");
        headers.insert::<USN>("uuid:ae239f00-ae2b-bbad-ddf1-88ddcc00a234::upnp:rootdevice");
        
        let message = AliveMessage::new(headers).unwrap();

        // TODO: NOT FINISHED WITH THIS TEST YET
    }
}