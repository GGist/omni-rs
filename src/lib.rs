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


pub type SimpleResult<T> = Result<T, SimpleError>;
pub type SOAPResult<T> = Result<T, SOAPError>;



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

