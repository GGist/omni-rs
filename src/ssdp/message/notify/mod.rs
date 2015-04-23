
mod alive;

pub use self::alive::{AliveMessage, AliveVersion, AliveExtV11, AliveExtV20};

/*
/// A notify message that was sent by some device on the network.
pub enum NotifyMessage {
    /// Device recently joined or is renewing its status on the network.
    Alive(AliveMessage),
    /// Device updated one or more of it's devices/services on the network.
    Update(UpdateMessage),
    /// Device is gracefully removing itself from the network.
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


pub struct NotifyMessage {
    interface: Interface,
    notify_type: NTS,
    device_location: Url,
    server_info: String,
    header: Headers
}

impl NotifyMessage {
    pub fn new(headers: Headers) -> SSDPResult<NotifyMessage> {
        let nt_field = try!(headers.get::<NT>().ok_or(
            SSDPError::MissingHeader(NT.header_name())
        ));
        
        
    }
    
    pub fn version() -> NotifyVersion {
        
    }
}


NOTIFY * HTTP/1.1
HOST: 239.255.255.250:1900
NT: upnp:rootdevice
NTS: ssdp:alive
USN: uuid:de305d54-75b4-431b-adb2-eb6b9e546013::upnp:rootdevice
BOOTID.UPNP.ORG: 89
CONFIGID.UPNP.ORG: 8*/