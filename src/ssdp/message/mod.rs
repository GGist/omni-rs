use hyper::http::{self};
/*
use {SSDPError};
use super::notify::{NotifyMessage};
use super::search::{SearchRequest, SearchResponse};*/

pub mod notify;

const UPNP_10_VERSION_NAME: &'static str = "UPnP/1.0";
const UPNP_11_VERSION_NAME: &'static str = "UPnP/1.1";
const UPNP_20_VERSION_NAME: &'static str = "UPnP/2.0";

const VALID_SEARCH_RESPONSE_CODE: u16 = 200;

const NOTIFY_HEADER: &'static str = "NOTIFY";
const SEARCH_HEADER: &'static str = "M-SEARCH";

/// A trait for messages that allow vendor specific information to be exposed.
pub trait MessageExt {
    /// Checks to see if the header has a field-name that matches the given
    /// name and returns all field-values that match that name or none.
    fn check_header(&self, name: &str) -> Option<&[Vec<u8>]>;
}
/*
/// Enumerates the types of SSDP messages.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SSDPMessage {
    /// An asynchronous notify message.
    NotifyMessage(NotifyMessage),
    /// A synchronous search request.
    SearchRequest(SearchRequest),
    /// A synchronous search response.
    SearchResponse(SearchResponse)
}

impl SSDPMessage {
    /// Create a new SSDPMessage from the given message.
    pub fn new(request: Vec<u8>) -> SSDPResult<SSDPMessage> {
        if let Ok(n) = http::parse_request(&mut Cursor::new(&request[..])) {
            return ssdp_from_request(n)
        }
        
        if let Ok(n) = http::parse_response(&mut Cursor::new(&request[..]) {
            return ssdp_from_response(n)
        }
        
        Err(SSDPError::InvalidHttp(request))
    }
}

/// Parses the HTTP request message as an SSDP message, specifically either a
/// NOTIFY or M-SEARCH request.
fn ssdp_from_request(req: Incoming<(Method, RequestUri)>) -> SSDPResult<SSDPMessage> {
    match req.subject {
        (Method::Extension(n), RequestUri::Star) => {
            match &n[..] {
                NOTIFY_HEADER => Ok(SSDPMessage::NotifyMessage(try!(NotifyMessage::new(req.headers)))),
                SEARCH_HEADER => Ok(SSDPMessage::SearchRequest(try!(SearchRequest::new(req.headers)))),
                _ => Err(SSDPError::InvalidMethod(n))
            }
        },
        (n, RequestUri::Star)            => Err(SSDPError::InvalidMethod(n.to_string())),
        (_, RequestUri::AbsolutePath(n)) => Err(SSDPError::InvalidUri(n)),
        (_, RequestUri::Authority(n))    => Err(SSDPError::InvalidUri(n)),
        (_, RequestUri::AbsoluteUri(n))  => Err(SSDPError::InvalidUri(n.serialize()))
    }
}

/// Parses the HTTP response message as an SSDP message, specifically an M-SEARCH
/// response, since that is the only type of HTTP response that is valid for SSDP.
fn ssdp_from_response(req: Incoming<RawStatus>) -> SSDPResult<SSDPMessage> {
    if req.subject.0 != VALID_SEARCH_RESPONSE_CODE {
        Err(SSDPError::ResponseCode(req.subject.0))
    } else {
        Ok(SSDPMessage::SearchResponse(try!(SearchResponse::new(req.headers))))
    }
}

*/