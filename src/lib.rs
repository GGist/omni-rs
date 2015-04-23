#![feature(lookup_host, udp, libc, collections)]

use std::error::{Error};
use std::fmt::{self, Display, Formatter};

extern crate hyper;
extern crate libc;
extern crate time;
extern crate url;

pub mod forum;
pub mod net;
pub mod ssdp;
pub mod util;
pub mod version;
pub mod vendor;

pub type SSDPResult<T> = Result<T, SSDPError>;
pub type SimpleResult<T> = Result<T, SimpleError>;
pub type SOAPResult<T> = Result<T, SOAPError>;

/// Enumerates all errors that can occur when dealing with an SSDP message.
#[derive(Debug)]
pub enum SSDPError {
    /// Message is not valid HTTP.
    ///
    /// Message is supplied as a list of bytes.
    InvalidHttp(Vec<u8>),
    /// Message consists of an error code that is not 200.
    ///
    /// Error code is supplied.
    ResponseCode(u16),
    /// Method supplied is not a valid SSDP method.
    ///
    /// Method received is supplied.
    InvalidMethod(String),
    /// Uri supplied is not a valid SSDP uri.
    ///
    /// URI received is supplied.
    InvalidUri(String),
    /// Header is missing from the message.
    ///
    /// Expected header is supplied.
    MissingHeader(&'static str),
    /// Header has an invalid value.
    ///
    /// Header name with error message are supplied.
    InvalidHeader(&'static str, &'static str),
    /// Some other error occurred.
    Other(Box<Error>)
}

impl Display for SSDPError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            SSDPError::InvalidHttp(ref n) => {
                let http_str = String::from_utf8_lossy(n);
                
                f.write_fmt(format_args!("Invalid Http: {}", http_str))
            },
            SSDPError::ResponseCode(n) => {
                f.write_fmt(format_args!("Response Code: {}", n))
            },
            SSDPError::InvalidMethod(ref n) => {
                f.write_fmt(format_args!("Invalid Method: {}", n))
            },
            SSDPError::InvalidUri(ref n) => {
                f.write_fmt(format_args!("Invalid URI: {}", n))
            },
            SSDPError::MissingHeader(n) => {
                f.write_fmt(format_args!("Missing Header: {}", n))
            },
            SSDPError::InvalidHeader(name, value) => {
                f.write_fmt(format_args!("Invalid Header: {}: {}", name, value))
            },
            SSDPError::Other(ref n) => {
                f.write_fmt(format_args!("Other: {}", n.description()))
            }
        }
    }
}

/// Error containing just an error message.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimpleError {
    desc: &'static str
}

impl SimpleError {
    fn new(desc: &'static str) -> SimpleError {
        SimpleError{ desc: desc }
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.desc)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        self.desc
    }
}

/// Enumerates all errors that can occure when dealing with a SOAP message.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SOAPError {
    None
}

