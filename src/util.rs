//! Utility functions and constants used throughout the crate.

use std::io::{ErrorKind, Error, Result};
use std::net::{self, Ipv4Addr, UdpSocket, SocketAddr};

const UNUSED_PORT_START: u16 = 1024;
const UNUSED_PORT_END: u16 = 49151;

/// Returns a list of all local IPv4 Addresses.
pub fn ipv4_net_addrs() -> Result<Vec<Ipv4Addr>> {
    let sock_iter = try!(net::lookup_host(""));
    
    let ipv4_list = sock_iter.filter_map(|addr|
        match addr {
            Ok(SocketAddr::V4(n)) => Some(*n.ip()),
            _                     => None
        }
    ).collect();
    
    Ok(ipv4_list)
}

/// Try to bind to a UDP port within the minimum and maximum port range.
pub fn try_bind_udp(ip: Ipv4Addr) -> Result<UdpSocket> {
    try_range_udp(ip, UNUSED_PORT_START, UNUSED_PORT_END).map_err( |_|
        Error::new(ErrorKind::Other, "Could Not Bind To Any Ports")
    )
}

/// Try to bind to a UDP port within the range [start,end].
pub fn try_range_udp(ip: Ipv4Addr, start: u16, end: u16) -> Result<UdpSocket> {
    if start < UNUSED_PORT_START || start > UNUSED_PORT_END {
        return Err(Error::new(ErrorKind::Other, "Start Port Range Is Not In Bounds [1024,49151]"))
    } else if end < UNUSED_PORT_START || end > UNUSED_PORT_END {
        return Err(Error::new(ErrorKind::Other, "End Port Range Is Not In Bounds [1024,49151]"))
    }
    
    for i in start..(end + 1) {
        if let Ok(udp_sock) = UdpSocket::bind((ip, i)) {
            return Ok(udp_sock)
        }
    }
    
    Err(Error::new(ErrorKind::Other, "Could Not Bind To A Port Within The Range Specified"))
}