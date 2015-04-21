use std::fmt::{Formatter, Display, Result};

use hyper::header::{HeaderFormat, Header};

use ssdp::{FieldPair};

const ST_HEADER_NAME: &'static str = "ST";

const ST_ALL_VALUE: &'static str = "ssdp:all";

/// Represents an ST header which specifies the Search Target.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ST {
    All,
    Target(FieldPair)
}

unsafe impl Sync for ST { }

unsafe impl Send for ST { }

impl Header for ST {
    fn header_name() -> &'static str {
        ST_HEADER_NAME
    }
    
    fn parse_header(raw: &[Vec<u8>]) -> Option<Self> {
        if raw.len() != 1 {
            return None
        }
        
        if &raw[0][..] == ST_ALL_VALUE.as_bytes() {
            Some(ST::All)
        } else {
            FieldPair::new(&raw[0][..]).map( |x| ST::Target(x) )
        }
    }
}

impl HeaderFormat for ST {
    fn fmt_header(&self, fmt: &mut Formatter) -> Result {
        match *self {
            ST::All           => try!(fmt.write_str(ST_ALL_VALUE)),
            ST::Target(ref n) => try!(Display::fmt(n, fmt))
        };
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Header};
    
    use ssdp::{FieldPair};
    use super::{ST};
    
    #[test]
    fn positive_all() {
        let st_all_header = &[b"ssdp:all"[..].to_vec()];
    
        match ST::parse_header(st_all_header) {
            Some(ST::All) => (),
            _ => panic!("Failed To Match ST::All Header")
        }
    }
    
    #[test]
    fn positive_field_upnp() {
        let st_upnp_root_header = &[b"upnp:some_identifier"[..].to_vec()];
    
        match ST::parse_header(st_upnp_root_header) {
            Some(ST::Target(FieldPair::UPnP(_))) => (),
            _ => panic!("Failed To Match ST::Target Header To FieldPair::UPnP")
        }
    }
    
    #[test]
    fn positive_field_urn() {
        let st_urn_root_header = &[b"urn:some_identifier"[..].to_vec()];
    
        match ST::parse_header(st_urn_root_header) {
            Some(ST::Target(FieldPair::URN(_))) => (),
            _ => panic!("Failed To Match ST::Target Header To FieldPair::URN")
        }
    }
    
    #[test]
    fn positive_field_uuid() {
        let st_uuid_root_header = &[b"uuid:some_identifier"[..].to_vec()];
    
        match ST::parse_header(st_uuid_root_header) {
            Some(ST::Target(FieldPair::UUID(_))) => (),
            _ => panic!("Failed To Match ST::Target Header To FieldPair::UUID")
        }
    }
}