/// A notify message that adheres to the UPnP 1.0 standard.
///
/// This message encapsulates information pertaining to an announcement made by
/// a device on the network advertising either an exposed device or service.
pub enum NotifyMessage {
    Alive(AliveMessage),
    Update(UpdateMessage),
    ByeBye(ByeByeMessage)
}

impl NotifyMessage {
    fn new(headers: Headers) -> SSDPResult<NotifyMessage> {
        match headers.get::<NTS>() {
            Some(NTS::Alive)  => NotifyMessage::Alive(AliveMessage::new(headers)),
            Some(NTS::Update) => NotifyMessage::Update(UpdateMessage::new(headers)),
            Some(NTS::ByeBye) => NotifyMessage::ByeBye(ByeByeMessage::new(headers)),
            None => SSDPError::InvalidHeader(NTS.header_name(), "Valid Header Value Not Found")
        }
    }
    
    fn get_nt() ->
    
    fn get_usn
}


/// A notify message trait for inspecting fields introduced in the UPnP 1.1 standrd.
pub trait NotifyExtV11 {
    pub fn boot_id(&self) -> u32;
    
    pub fn config_id(&self) -> u32;
    
    pub fn search_port(&self) -> Option<u16>;
}

/// A notify message trait for inspecting fields introduced in the UPnP 2.0 standrd.
pub trait NotifyExtV20: NotifyExtV11 {
    pub fn next_boot_id(&self) -> u32;
}

/// A type that exposes the version information for a particular notify message.
///
/// Since the base interface for a notify message is considered compliant with
/// the UPnP 1.0 standard, the V10 variant does not need to expose any more info.
pub enum NotifyVersion {
    V10,
    V11(Box<NotifyExtV11>),
    V20(Box<NotifyExtV20>)
}

impl NotifyVersion {
    fn new(server: &str) -> SSDPResult<NotifyVersion> {
        
    }
}

struct NotifyExtV11Impl {

}

impl NotifyExtV11 for NotifyExtV11Impl {
    pub fn boot_id() -> 
    
    pub fn config_id() ->
    
    pub fn next_boot_id() -> 
    
    pub fn search_port() -> Option<> {
    
    }
}

pub struct NotifyExtV20 {
    
}

NOTIFY * HTTP/1.1
HOST: 239.255.255.250:1900
NT: upnp:rootdevice
NTS: ssdp:alive
USN: uuid:de305d54-75b4-431b-adb2-eb6b9e546013::upnp:rootdevice
BOOTID.UPNP.ORG: 89
CONFIGID.UPNP.ORG: 8