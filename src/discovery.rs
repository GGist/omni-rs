use uuid::{UUID};

/// Subscribe address for all NOTIFY messages broadcast on a network.
const MULTICAST_LISTEN: (u8, u8, u8, u8, u16) = (239, 255, 255, 250, 1900);

/// Stream that receives NotifyMessage alerts from UPnP enabled interfaces
/// across one or more network interfaces.
///
/// Each DiscoveryStream object will spawn a thread to listen for messages on.
struct DiscoveryStream {
    exit_thread: Arc<AtomicBool>
}

impl DiscoveryStream {
    /// Listen an all IPv4 network interfaces available.
    ///
    /// Will make no attempt to detect IPv4 interfaces on the same subnet. If
    /// you have multiple network interfaces that are on the same subnet, you
    /// will receive duplicate messages.
    fn all_interfaces<T>(message_handler: T) -> Result<DiscoveryStream>
        where T: FnMut(DiscoveryMessage) {
        let interfaces = try!(util::ipv4_net_addrs().map_err( |e|
            Error::new(Other, "Failed To Get Host Addresses", None)
        ));
        
        with_interfaces(message_handler, &interfaces[..])
    }
    
    /// Listen on a set of IPv4 network interfaces.
    ///
    /// If any of the specified interfaces are unavailable, an error will be
    /// returned. For information on interfaces on the same subnet, see
    /// DiscoveryStream::all_interfaces.
    fn with_interfaces<T>(message_handler: T, addrs: &[Ipv4Addr]) -> Result<DiscoveryStream>
        where T: FnMut(DiscoveryMessage) {
        let interfaces = Vec::with_capacity(addrs.len());
        let exit_flag = Arc::new(AtomicBool::new(false));
    
        // Instantiate all UDP sockets for listening on.
        for i in with_interfaces {
            let udp_sock = try!(util::try_bind_udp(i));
        
            interfaces.push(udp_sock);
        }
        
        // Spawn thread to listen for UPnP notify messages.
        let exit_flag_copy = exit_thread.clone();
        thread::spawn(move || {
            discovery_listen(message_handler, interfaces, exit_flag_copy);
        });
        
        Ok(DiscoveryStream{ exit_thread: exit_flag })
    }
}

/// Listens for UPnP notify messages on a series of UdpSockets and forward them
/// on to the sepcified message handler.
fn discovery_listen<T>(message_handler: T, sockets: Vec<UdpSocket>, exit_flag: Arc<AtomicBool>)
    where T: FnMut(DiscoveryMessage) {
    
    
    
    
}