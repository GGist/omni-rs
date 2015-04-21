const UPNP_10_VERSION_NAME: &'static str = "UPnP/1.0";
const UPNP_11_VERSION_NAME: &'static str = "UPnP/1.1";
const UPNP_20_VERSION_NAME: &'static str = "UPnP/2.0";

pub struct AliveMessage {
    headers: Headers,
    created: PreciseTime,
    version: AliveVersion,
    max_age: Duration,
    query:   QueryType,
    uuid:    Vec<u8>
}

impl AliveMessage {
    fn new(headers: Headers) -> SSDPResult<AliveMessage> {
        let max_age = try!(first_max_age(&headers));
        
        let url = try!(location_as_url(&headers));
        let uuid = try!(uuid_as_bytes(&headers).to_vec());
        
        let version = try!(AliveVersion::new(&headers));
        let query = try!(generate_query(&headers));
        
        Ok(AliveMessage{ headers: headers, created: PreciseTime::now(),
            version: version, max_age: max_age, query: query, uuid: uuid})
    }
    
    pub fn is_expired(&self) -> bool {
        self.created.to(PreciseTime::now()) > self.max_age
    }
    
    pub fn max_age(&self) -> Duration {
        self.max_age
    }
    
    pub fn uuid(&self) -> &[u8] {
        &self.uuid[..]
    }
    
    pub fn version(&self) -> AliveVersion {
        self.version
    }
    
    pub fn query(&self) -> QueryType<'a> {
        
    }
}

/// Returns the uuid portion of the URN header as bytes.
fn uuid_as_bytes(header: &Headers) -> SSDPResult<&[u8]> {
    let uuid = try!(headers.get::<USN>().ok_or(
        SSDPResult::MissingHeader(USN.header_name())
    )).0;
    
    match uuid {
        FieldPair::UUID(ref n) => Ok(n[..]),
        _ => Err(SSDPResult::InvalidHeader(
                    USN.header_name(), "UUID Not Found As First Value"
                ))
    }
}

/// Returns the location header field as a Url.
fn location_as_url(headers: &Headers) -> SSDPResult<Url> {
    let location = try!(headers.get::<Location>().ok_or(
        SSDPResult::MissingHeader(Location.header_name())
    ));
    
    Url::parse(location).map_err(|e|
        SSDPError::InvalidHeader(
            Location.header_name(), "Could Not Parse Location As A Url"
        )
    )
}

/// Returns first max-age directive found in the cache-control header.
fn first_max_age(headers: &Headers) -> SSDPResult<u32> {
    let cache_control_list = try!(headers.get::<CacheControl>().ok_or(
        SSDPResult::MissingHeader(CacheControl.header_name())
    ));
    
    // Return First Max-Age Directive Found, Ignore Duplicates
    for i in cache_control_list.iter() {
        if let CacheDirective::MaxAge(n) = i {
            return n
        }
    }
}

impl MessageExt for AliveMessage {
    fn check_header(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.headers.get_raw(name)
    }
}

/// Alive message versions for different UPnP versions.
pub enum AliveVersion {
    V10,
    V11(Box<AliveExtV11>),
    V20(Box<AliveExtV20>)
}

impl AliveVersion {
    /// Create a new AliveVersion object.
    fn new(headers: &Headers) -> SSDPResult<AliveVersion> {
        let server = try!(headers.get::<Server>().ok_or(
            SSDPError::MissingHeader(Server.header_name())
        ));
        
        if server.contains(UPNP_10_VERSION_NAME) {
            Ok(AliveVersion::V10)
        } else if server.contains(UPNP_11_VERSION_NAME) {
            let alive_ext = try!(AliveExtV11Impl::new(headers));
            
            Ok(AliveVersion::V11(Box::new(alive_ext) as Box<AliveExtV11>))
        } else if server.contains(UPNP_20_VERSION_NAME) {
            let alive_ext = try!(AliveExtV20Impl::new(headers));
            
            Ok(AliveVersion::V20(Box::new(alive_ext) as Box<AliveExtV20>))
        } else {
            Err(SSDPError::InvalidHeader(Server.header_name(), 
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

pub struct AliveExtV11Impl {
    boot_id:   u32,
    config_id: u32,
    port:      Option<u16>
}

impl AliveExtV11Impl {
    fn new(header: &Headers) -> SSDPResult<AliveExtV11Impl> {
        let boot_id = try!(header.get::<BOOTID>().ok_or(
            SSDPResult::MissingHeader(BOOTID.header_name())
        ));
        
        let config_id = try!(header.get::<CONFIGID>().ok_or(
            SSDPResult::MissingHeader(CONFIGID.header_name())
        ));
        
        let port = header.get::<SEARCHPORT>().map(|n| *n);
        
        Ok(AliveExtV11Impl{ boot_id: *boot_id, config_id: *config_id, port: port })
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
    fn secure_location(&self) -> Option<String>;
}

pub struct AliveExtV20Impl {
    parent:     AliveExtV11Impl
    secure_loc: Option<String>
}

impl AliveExtV20Impl {
    fn new(header: &Headers) -> SSDPResult<AliveExtV20Impl> {
        let parent = try!(AliveExtV11Impl::new(header));
        
        let secure_loc = header.get::<SECURELOCATION>().map(|n| *n);
        
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
        self.secure_loc.map(|n| &n)
    }
}