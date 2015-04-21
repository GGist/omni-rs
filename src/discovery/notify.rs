/// Interface schema standardized by the UPnP Forum.
const UPNP_FORUM_SCHEMA: &'static str = "schemas-upnp-org";

/// Mandatory header fields (UPnP 1.0).
const NOTIFY_METHOD:    &'static str = "NOTIFY";
const NT_HEADER:        &'static str = "NT";
const NTS_HEADER:       &'static str = "NTS";
const USN_HEADER:       &'static str = "USN";

/// Allowed header fields (UPnP 2.0).
const BOOT_ID_HEADER:   &'static str = "BOOTID.UPNP.ORG";
const CONFIG_ID_HEADER: &'static str = "CONFIGID.UPNP.ORG";
const SEARCH_PORT_HEADER:     &'static str = "SEARCHPORT.UPNP.ORG";
const SECURE_LOCATION_HEADER: &'static str = "SECURELOCATION.UPNP.ORG";

/// Parsing identifiers.
const USN_UUID_IDENT: &'static str = "uuid:";
const URN_UUID_IDENT: &'static str = "urn:";

/// Parsing lengths.
const USN_UUID_LEN: usize = 36;


/// Type of notification received from some interface.
///
/// Because of the way UPnP works, all devices will be advertised as both a
/// NestedDevice and a UniqueDevice. The root device will also be advertised as
/// a RootDevice in addition to the previous two types. Because of this caveat,
/// if you only want to see each device once per notify (instead of two or three
/// times) you should only care about NestedDevice types.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum NotifyType<'a> {
    /// Signifies a root device.
    RootDevice,
    /// Signifies some device.
    UniqueDevice,
    /// Signifies a nested device.
    NestedDevice(DeviceType, Version),
    /// Signifies a nested service.
    /// Note that the UUID is of the enclosing device.
    NestedService(ServiceType, Version)
}

/// Signifies the purpose of the discovery message.
#[derive(Copy, PartialEq, Eq, Hash, Debug)]
enum NotifySubType {
    /// Interface was just added to the network.
    Alive,
    /// Interface is updating/renewing their status on the network.
    Update,
    /// Interface is removing itself from the network.
    ByeBye
}

/// Notification message sent by an interface on the network because it has
/// changed it's status in some way (see NotifySubType).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct NotifyMessage {
    type: NotifyType,
    sub_type: NotifySubType,
    location: String,
    server: String,
    valid: Duration,
    uuid: UUID,
    schema: SchemaType
}

impl NotifyMessage {
    fn from_message(http_msg: String) -> SSDPResult<NotifyMessage> {
        let dummy_socket = SocketAddr::new(IpAddr::new_v4(127, 0, 0, 1), 1900);
        let http_request = try!(Request::new(&http_msg), dummy_socket);
        
        try!(validate_notify_method(&http_request));
        try!(validate_nt_with_usn(&http_request));
        
        
    }
    
    pub fn server_info(&self) -> &str {
        &self.server[..]
    }
}

fn validate_notify_method(req: &Request) -> SSDPResult<()> {
    match req.method {
        Extension(ref n) if n == NOTIFY_METHOD
            => Ok(n),
        _   => Err(SSDPError::SSDPMethodError)
    }
}


fn validate_nt_with_usn(req: &Request) -> SSDPResult<()> {
    let nt_header = try!(req.headers.get_raw(NT_HEADER).ok_or(
        SSDPError::MissingHeader(NT_HEADER)
    ));
    let usn_header = try!(req.headers.get_raw(USN_HEADER).ok_or(
        SSDPError::MissingHeader(USN_HEADER)
    ));
    
    // One Instance Of Both Headers, NT Is Prefix Of USN
    match (nt_header.len() != 1, usn_header.len() != 1) {
        (true, _) => Err(SSDPError::InvalidHeader(NT_HEADER, "NT Header Does Not Appear Once")),
        (_, true) => Err(SSDPError::InvalidHeader(USN_HEADER, "USN Header Does Not Appear Once")),
        _ => {
            for (a, b) in nt_header[0].iter().zip(usn_header[0].iter()) {
                if a != b {
                    return Err(SSDPError::InvalidHeader(USN_HEADER, 
                        "USN Header Is Not Prefixed With NT Header"
                    ))
                }
            }
        
            Ok(())
        }
    }
}

fn parse_usn(req: &Request) -> SSDPResult<(NotifyType, UUID)> {
    let usn_header = try!(req.headers.get_raw(USN_HEADER).ok_or(
        SSDPError::MissingHeader(USN_HEADER)
    ));
    let usn_bytes = &usn_header[0][..];
    let usn_bytes_iter = usn_bytes.iter();
    
    // Parse UUID Portion
    let uuid = try!(parse_uuid(&mut usn_bytes_iter));
    
    // Parse Field Separator
    match (usn_bytes_iter.next(), usn_bytes_iter.next()) {
        (Some(':' as u8), Some(':' as u8)) => (),
        _ => return Err(SSDPError::InvalidHeader(USN_HEADER, 
            "Invalid Or Missing USN Field Separator"))
    };
    
    // Parse URN For NotifyType
    let notify_type = try!(parse_usn(&mut usn_bytes_iter));
}

fn parse_uuid<I>(usn_bytes_iter: &mut I) -> SSDPResult<UUID>
    where I: Iterator<Item=u8> {
    let uuid_bytes = [0u8; USN_UUID_LEN];
    
    // Validate uuid Identifier
    let mut usn_iter = USN_UUID_IDENT.iter();
    for (a, b) in usn_bytes_iter.zip(usn_iter) {
        if a != b {
            return Err(SSDPError::InvalidHeader(USN_HEADER, "uuid Identifier Invalid"))
        }
    }
    
    // Check if Identifier Ended Early
    if usn_iter.next().is_some() {
        return Err(SSDPError::InvalidHeader(USN_HEADER, "uuid Identifier Too Short"))
    }
    
    // Pull Out uuid Value
    for i in 0..uuid_bytes.len() {
        match i.next() {
            Some(n) => uuid_bytes[i] = n,
            None    => return Err(SSDPError::InvalidHeader(USN_HEADER, "uuid Value Too Short"))
        }
    }
    
    // Build UUID Struct
    UUID::from_bytes(&uuid_bytes[..]).map_err( |_|
        SSDPError::InvalidHeader(USN_HEADER, "uuid Value Invalid")
    )
}

fn parse_urn(usn_bytes_iter: &mut I) -> SSDPResult<NotifyType>
    where I: Iterator<Item=u8> {
    if usn_bytes_iter()
}