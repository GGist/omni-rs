use std::error::{Error};
use std::fmt::{self, Debug, Formatter};

use hyper::header::{Location, Server, CacheDirective, CacheControl, Host, 
                    Headers, Header, HeaderFormat};
use time::{Duration, PreciseTime};
use url::{Url};

use {SSDPError, SSDPResult};
use forum::{GenericQuery, QueryType, TargetType};
use ssdp::{FieldPair};
use ssdp::header::{HeaderView, SearchPort, SecureLocation, BootID, NT, USN, ConfigID};
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
    headers:  Headers,
    created:  PreciseTime,
    version:  AliveVersionImpl,
    max_age:  Duration,
    target:   TargetType,
    location: Url
}

impl AliveMessage {
    /// Create a new AliveMessage from the given header.
    fn new(headers: Headers) -> SSDPResult<AliveMessage> {
        let (version, duration, url, target) = try!(alive_pieces(&headers));
        
        Ok(AliveMessage{ headers: headers, created: PreciseTime::now(),
            version: version, max_age: duration, target: target, location: url })
    }
    
    /// Whether or not the cache control for this message has expired.
    pub fn is_expired(&self) -> bool {
        self.created.to(PreciseTime::now()) > self.max_age
    }
    
    /// Maximum time for which this alive message is valid unless a shutdown
    /// (ByeByeMessage) is received prior.
    ///
    /// This is the initial duration sent, not the duration left (use is_expired).
    pub fn max_age(&self) -> Duration {
        self.max_age
    }
    
    /// Contains environment information from which the message originated.
    /// Message will be in the form "OS/version UPnP/version product/version".
    pub fn server_info(&self) -> &str {
        // We Processed The Server Header Field When Looking At The Message
        // Version. We Can Therefore Safely Unwrap It Out Of The Header.
        &self.headers.view::<Server>().unwrap().0[..]
    }
    
    /// UPnP version information containing additional fields for clients to utilize.
    pub fn version<'a>(&'a self) -> AliveVersion<'a> {
        match self.version {
            AliveVersionImpl::V10 => AliveVersion::V10,
            AliveVersionImpl::V11(ref n) => AliveVersion::V11(n),
            AliveVersionImpl::V20(ref n) => AliveVersion::V20(n)
        }
    }
    
    /// Query object associated with this message for obtaining a remote object
    /// that can be used to invoke remote procedure calls on the device.
    pub fn query<'a>(&'a self) -> QueryType<'a> {
        //let udn = 
        panic!("TODO")
        //&self.query
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
        
        try!(f.write_str(", target: "));
        try!(Debug::fmt(&self.target, f));
        
         try!(f.write_str(", location: "));
        try!(Debug::fmt(&self.location, f));
        
        f.write_str(" }")
    }
}

/// Delegate for the creation process of an alive message.
fn alive_pieces<T>(headers: T) -> SSDPResult<(AliveVersionImpl, Duration, Url, TargetType)>
    where T: HeaderView {
    // Extract Required Headers
    let ref host_name = try!(try_view_header::<T, Host>(&headers)).hostname;
    let ref cache_control = try!(try_view_header::<T, CacheControl>(&headers)).0;
    let ref location = try!(try_view_header::<T, Location>(&headers)).0;
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

/// Attempts to get a reference to a header value H.
/// Returns a reference to H or an error if it does not exist in headers.
fn try_view_header<'a, T, H>(headers: &'a T) -> SSDPResult<&'a H>
    where T: HeaderView, H: Header + HeaderFormat {
    headers.view::<H>().ok_or(SSDPError::MissingHeader(H::header_name()))
}

/// Returns an error if the host field does not match the standard multicast address.
fn check_multicast_host(host_name: &str) -> SSDPResult<()> {
    if host_name != message::MESSAGE_MULTICAST_HOST {
        Err(SSDPError::InvalidHeader(Host::header_name(),
            "Host Field Contains Wrong Multicast Address"))
    } else { 
        Ok(()) 
    }
}

/// Returns an error if the rules for the relationship between the NT and USN
/// fields are not adhered to.
fn check_nt_usn_rules(notify_type: &FieldPair, usn_uuid: &FieldPair,
                      usn_type: &Option<FieldPair>) -> SSDPResult<()> {
    // Verify That The First Portion Of USN Is A UUID
    let uuid = match *usn_uuid {
        FieldPair::UUID(ref n) => &n[..],
        _ => return Err(SSDPError::InvalidHeader(USN::header_name(),
                        "USN Field Does Not Start With A UUID"))
    };
    
    // Verify The Relationship Between NT And USN
    match *notify_type {
        FieldPair::UUID(ref n) => {
            if uuid != &n[..] {
                Err(SSDPError::InvalidHeader(NT::header_name(),
                    "UUID Does Not Match UUID In USN Field"))
            } else if usn_type.is_some() {
                Err(SSDPError::InvalidHeader(USN::header_name(),
                    "Second Field Should Be Empty When NT Contains UUID"))
            } else { Ok(()) }
        },
        ref n => {
            // Unwrap The Second Field Of USN And Match Against NT
            if let Some(ref n) = *usn_type {
                compare_nt_usn(notify_type, n)
            } else {
                Err(SSDPError::InvalidHeader(USN::header_name(),
                    "Second Field Is Empty When It Should Contain NT"))
            }
        }
    }
}

/// Apply rules to NT and second field of USN to check if they are both URN or
/// UPnP variants, and if so, compare their values.
/// Return an error if NT and second field of USN are not equal.
fn compare_nt_usn(notify_type: &FieldPair, usn_type: &FieldPair) -> SSDPResult<()> {
    match (notify_type, usn_type) {
        (&FieldPair::URN(ref n), &FieldPair::URN(ref u)) => {
            if n != u {
                Err(SSDPError::InvalidHeader(NT::header_name(),
                    "NT URN Value Does Not Match USN URN Value"))
            } else { Ok(()) }
        },
        (&FieldPair::UPnP(ref n), &FieldPair::UPnP(ref u)) => {
            if n != u {
                Err(SSDPError::InvalidHeader(NT::header_name(),
                    "NT UPnP Value Does Not Match USN UPnP Value"))
            } else { Ok(()) }
        },
        _ => Err(SSDPError::InvalidHeader(NT::header_name(),
                 "Either NT Is Unknown Or It Did Not Match Second Field Of USN"))
    }
}

/// Returns the first max-age directive found in the list.
fn first_max_age(directives: &[CacheDirective]) -> SSDPResult<u32> {
    // Return First Max-Age Directive Found, Ignore Duplicates
    for i in directives.iter() {
        if let &CacheDirective::MaxAge(n) = i {
            return Ok(n)
        }
    }
    
    Err(SSDPError::InvalidHeader(CacheControl::header_name(),
        "No Max-Age Found"))
}

/// Returns the location as a str into a Url object.
fn location_as_url(location: &str) -> SSDPResult<Url> {
    Url::parse(location).map_err(|e|
        SSDPError::InvalidHeader(
            Location::header_name(), "Could Not Parse Location As A Url"
        )
    )
}

/// Returns the NT field pair as a TargetType.
fn notify_as_target(notify_type: &FieldPair) -> SSDPResult<TargetType> {
    match TargetType::new(notify_type) {
        Ok(n)  => Ok(n),
        Err(e) => Err(SSDPError::Other(Box::new(e) as Box<Error>))
    }
}

impl MessageExt for AliveMessage {
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
        let server = try!(try_view_header::<T, Server>(&headers));
        
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
    fn new<T>(headers: T) -> SSDPResult<AliveExtV11Impl> where T: HeaderView {
        let boot_id = try!(try_view_header::<T, BootID>(&headers)).0;
        let config_id = try!(try_view_header::<T, ConfigID>(&headers)).0;
        
        let port = headers.view::<SearchPort>().map(|n| n.0);
        
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
    /*
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
    }*/
}