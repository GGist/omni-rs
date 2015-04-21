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

pub use self::bootid::BOOTID;
pub use self::configid::CONFIGID;
pub use self::man::MAN;
pub use self::mx::MX;
pub use self::nt::NT;
pub use self::nts::NTS;
pub use self::searchport::SEARCHPORT;
pub use self::securelocation::SECURELOCATION;
pub use self::st::ST;
pub use self::usn::USN;