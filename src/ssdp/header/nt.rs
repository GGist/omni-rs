use std::fmt::{Formatter, Display, Result};

use hyper::header::{HeaderFormat, Header};

use ssdp::{FieldPair};

const NT_HEADER_NAME: &'static str = "NT";

/// Represents an NT header which specifies a Notification Type.
///
/// Any double colons will not be processed as separate FieldPairs.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NT(FieldPair);

impl NT {
    pub fn new(field: FieldPair) -> NT {
        NT(field)
    }
}

unsafe impl Sync for NT { }

unsafe impl Send for NT { }

impl Header for NT {
    fn header_name() -> &'static str {
        NT_HEADER_NAME
    }
    
    fn parse_header(raw: &[Vec<u8>]) -> Option<Self> {
        if raw.len() != 1 {
            return None
        }
        
        match FieldPair::new(&raw[0][..]) {
            Some(n) => Some(NT(n)),
            None    => None
        }
    }
}

impl HeaderFormat for NT {
    fn fmt_header(&self, fmt: &mut Formatter) -> Result {
        try!(Display::fmt(&self.0, fmt));
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Header};
    
    use super::{NT};
    use ssdp::FieldPair::{UPnP, UUID, URN, Unknown};

    #[test]
    fn positive_uuid() {
        let uuid_header = &["uuid:a984bc8c-aaf0-5dff-b980-00d098bda247".to_string().into_bytes()];

        let data = match NT::parse_header(uuid_header) {
            Some(NT(UUID(n))) => n,
            _                 => panic!("uuid Token Not Parsed")
        };
        
        assert!(uuid_header[0][5..].iter().zip(data.iter()).all(|(a,b)| a == b));
    }
    
    #[test]
    fn positive_upnp() {
        let upnp_header = &["upnp:rootdevice".to_string().into_bytes()];
            
        let data = match NT::parse_header(upnp_header) {
            Some(NT(UPnP(n))) => n,
            _                 => panic!("upnp Token Not Parsed")
        };
        
        assert!(upnp_header[0][5..].iter().zip(data.iter()).all(|(a,b)| a == b));
    }
    
    #[test]
    fn positive_urn() {
        let urn_header = &["urn:schemas-upnp-org:device:printer:1".to_string().into_bytes()];
            
        let data = match NT::parse_header(urn_header) {
            Some(NT(URN(n))) => n,
            _                => panic!("urn Token Not Parsed")
        };
        
        assert!(urn_header[0][4..].iter().zip(data.iter()).all(|(a,b)| a == b));
    }
    
    #[test]
    fn positive_unknown() {
        let unknown_header = &["max-age:1500::upnp:rootdevice".to_string().into_bytes()];
            
        let (k, v) = match NT::parse_header(unknown_header) {
            Some(NT(Unknown(k, v))) => (k, v),
            _                       => panic!("Unknown Token Not Parsed")
        };
        
        let sep_iter = b":".iter();
        let mut original_iter = unknown_header[0][..].iter();
        let mut result_iter = k[..].iter().chain(sep_iter).chain(v[..].iter());
        
        assert!(original_iter.by_ref().zip(result_iter.by_ref()).all(|(&a,&b)| a == b));
        assert!(result_iter.next().is_none() && original_iter.next().is_none());
    }
    
    #[test]
    fn positive_short_field() {
        let short_header = &["a:a".to_string().into_bytes()];
        
        let (k, v) = match NT::parse_header(short_header) {
            Some(NT(Unknown(k, v))) => (k, v),
            _                       => panic!("Unknown Short Token Not Parsed")
        };
        
        let sep_iter = b":".iter();
        let mut original_iter = short_header[0][..].iter();
        let mut result_iter = k[..].iter().chain(sep_iter).chain(v[..].iter());
        
        assert!(original_iter.by_ref().zip(result_iter.by_ref()).all(|(&a,&b)| a == b));
        assert!(result_iter.next().is_none() && original_iter.next().is_none());
    }
    
    #[test]
    fn positive_leading_double_colon() {
        let leading_double_colon_header = &["uuid::a984bc8c-aaf0-5dff-b980-00d098bda247".to_string().into_bytes()];
        
        let result = match NT::parse_header(leading_double_colon_header).unwrap() {
            NT(UUID(n)) => n,
            _           => panic!("NT Double Colon Failed To Parse")
        };
        
        assert_eq!(result, b":a984bc8c-aaf0-5dff-b980-00d098bda247".to_vec());
    }
    
    #[test]
    #[should_panic]
    fn negative_double_colon() {
        let double_colon_header = &["::".to_string().into_bytes()];
        
        NT::parse_header(double_colon_header).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_single_colon() {
        let single_colon_header = &[":".to_string().into_bytes()];
        
        NT::parse_header(single_colon_header).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_empty_field() {
        let empty_header = &["".to_string().into_bytes()];
        
        NT::parse_header(empty_header).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_no_colon() {
        let no_colon_header = &["some_key-some_value".to_string().into_bytes()];
        
        NT::parse_header(no_colon_header).unwrap();
    }
}