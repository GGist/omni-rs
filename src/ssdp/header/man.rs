use std::fmt::{Formatter, Result};

use hyper::header::{HeaderFormat, Header};

const MAN_HEADER_NAME:  &'static str = "MAN";
const MAN_HEADER_VALUE: &'static str = "\"ssdp:discover\"";

/// Represents a MAN header which is used to define the scope of the M-SEARCH
/// HTTP extension.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MAN;

unsafe impl Sync for MAN { }

unsafe impl Send for MAN { }

impl Header for MAN {
    fn header_name() -> &'static str {
        MAN_HEADER_NAME
    }
    
    fn parse_header(raw: &[Vec<u8>]) -> Option<Self> {
        if raw.len() != 1 {
            return None
        }
        
        let man_bytes = MAN_HEADER_VALUE.as_bytes();
        match &raw[0][..] {
            n if n == man_bytes => Some(MAN),
            _ => None
        }
    }
}

impl HeaderFormat for MAN {
    fn fmt_header(&self, fmt: &mut Formatter) -> Result {
        try!(fmt.write_str(MAN_HEADER_VALUE));
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::{Header};
    
    use super::{MAN};
    
    #[test]
    fn positive_man() {
        let man_header = &[b"\"ssdp:discover\""[..].to_vec()];
        
        MAN::parse_header(man_header).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_wrong_case() {
        let wrong_case_man_header = &[b"\"SSDP:discover\""[..].to_vec()];
        
        MAN::parse_header(wrong_case_man_header).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_missing_quotes() {
        let missing_quotes_man_header = &[b"ssdp:discover"[..].to_vec()];
        
        MAN::parse_header(missing_quotes_man_header).unwrap();
    }
}