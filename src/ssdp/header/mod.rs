//! Headers and primitives for parsing headers within SSDP requests.
//!
//! This module combines abstractions at both the HTTPU/HTTPMU layer and SSDP
//! layer in order to provide a cleaner interface for extending the underlying
//! HTTP parsing library.

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

// TODO: Use trait Headers instead of hyper::header::Headers directly
// Provide implementation for hyper Headers as well as mock implementations
// for hashmaps with cfg(test) set in the impl
/*
pub trait Headers {
    fn set<H: Header + HeaderFormat>(&mut self, value: H);
    
    
}*/