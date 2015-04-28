//! Headers and primitives for parsing headers within SSDP requests.
//!
//! This module combines abstractions at both the HTTPU/HTTPMU layer and SSDP
//! layer in order to provide a cleaner interface for extending the underlying
//! HTTP parsing library.

use std::fmt::{Debug};

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

/// Trait for viewing the contents of a header structure.
pub trait HeaderView: Debug {
    /// View a reference to a header field if it exists.
    fn view<H>(&self) -> Option<&H> where H: Header + HeaderFormat;
    
    /// View a reference to the raw bytes of a header field if it exists.
    fn view_raw(&self, name: &str) -> Option<&[Vec<u8>]>;
}

impl<'a, T: ?Sized> HeaderView for &'a T where T: HeaderView {
    fn view<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
        HeaderView::view::<H>(*self)
    }
    
    fn view_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
        HeaderView::view_raw(*self, name)
    }
}

impl<'a, T: ?Sized> HeaderView for &'a mut T where T: HeaderView {
    fn view<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
        HeaderView::view::<H>(*self)
    }
    
    fn view_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
        HeaderView::view_raw(*self, name)
    }
}

impl HeaderView for Headers {
    fn view<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
        self.get::<H>()
    }
    
    fn view_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.get_raw(name)
    }
}

#[cfg(test)]
pub mod mock {
    use std::any::{Any};
    use std::borrow::{ToOwned};
    use std::collections::{HashMap};
    
    use hyper::header::{Header, HeaderFormat};
    
    use ssdp::header::{HeaderView};

    #[derive(Debug)]
    pub struct MockHeaderMap {
        map: HashMap<&'static str, (Box<Any>, [Vec<u8>; 1])>
    }
    
    impl MockHeaderMap {
        pub fn new() -> MockHeaderMap {
            MockHeaderMap{ map: HashMap::new() }
        }
        
        pub fn insert<H>(&mut self, value: &str) where H: Header + HeaderFormat {
            let header_bytes = [value.to_owned().into_bytes()];
            
            let header = match H::parse_header(&header_bytes[..]) {
                Some(n) => n,
                None    => panic!("Failed To Parse value As Header!!!")
            };
            
            self.map.insert(H::header_name(), (Box::new(header), header_bytes));
        }
    }
    
    impl HeaderView for MockHeaderMap {
        fn view<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
            match self.map.get(H::header_name()) {
                Some(&(ref header, _)) => header.downcast_ref::<H>(),
                None => None
            }
        }
        
        fn view_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
            match self.map.get(name) {
                Some(&(_, ref header_bytes)) => Some(header_bytes),
                None => None
            }
        }
    }
}