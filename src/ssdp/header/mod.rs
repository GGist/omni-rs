//! Headers and primitives for parsing headers within SSDP requests.
//!
//! This module combines abstractions at both the HTTPU/HTTPMU layer and SSDP
//! layer in order to provide a cleaner interface for extending the underlying
//! HTTP parsing library.

use std::collections::{HashMap};

use hyper::header::{Headers, Header, HeaderFormat};

mod bootid;
mod configid;
mod man;
mod mx;
mod nt;
mod nts;
mod searchport;
mod securelocation;
mod st;
mod usn;

pub use self::bootid::BootID;
pub use self::configid::ConfigID;
pub use self::man::Man;
pub use self::mx::MX;
pub use self::nt::NT;
pub use self::nts::NTS;
pub use self::searchport::SearchPort;
pub use self::securelocation::SecureLocation;
pub use self::st::ST;
pub use self::usn::USN;

/// Interface for objects that allow getting and setting of header values.
pub trait HeaderMap {
    /// Set a header field to the given value.
    fn set<H: Header + HeaderFormat>(&mut self, value: H);
    
    /// Get a reference to a header field if it exists.
    fn get<H: Header + HeaderFormat>(&self) -> Option<&H>;
    
    /// Get a mutable reference to a header field if it exists.
    fn get_mut<H: Header + HeaderFormat>(&mut self) -> Option<&mut H>;
    
    /// Returns true if a header field has been set, false otherwise.
    fn has<H: Header + HeaderFormat>(&self) -> bool;
    
    /// Remove a header field and returns true, false otherwise.
    fn remove<H: Header + HeaderFormat>(&mut self) -> bool;
}

impl HeaderMap for Headers {
    fn set<H: Header + HeaderFormat>(&mut self, value: H) {
        self.set(value)
    }
    
    fn get<H: Header + HeaderFormat>(&self) -> Option<&H> {
        self.get::<H>()
    }
    
    fn get_mut<H: Header + HeaderFormat>(&mut self) -> Option<&mut H> {
        self.get_mut::<H>()
    }
    
    fn has<H: Header + HeaderFormat>(&self) -> bool {
        self.has::<H>()
    }
    
    fn remove<H: Header + HeaderFormat>(&mut self) -> bool {
        self.remove::<H>()
    }
}
/*
#[cfg(test)]
impl<V> HeaderMap for HashMap<&'static str, V> where V: Header + HeaderFormat {
    fn set<H: Header + HeaderFormat>(&mut self, value: H) {
        
    }
    
    fn get<H: Header + HeaderFormat>(&self) -> Option<&H> {
        self.get::<H>()
    }
    
    fn get_mut<H: Header + HeaderFormat>(&mut self) -> Option<&mut H> {
        self.get_mut::<H>()
    }
    
    fn has<H: Header + HeaderFormat>(&self) -> bool {
        self.has::<H>()
    }
    
    fn remove<H: Header + HeaderFormat>(&mut self) -> bool {
        self.remove::<H>()
    }
}*/