//! Version numbering for devices and services.

pub const VERSION_1: u8 = 1;
pub const VERSION_2: u8 = 2;
pub const VERSION_3: u8 = 3;
pub const VERSION_4: u8 = 4;
pub const VERSION_5: u8 = 5;

/// Version numbers for devices and services.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
    V5
}

impl Version {
    /// Convert a string representation of a version to a Version.
    pub fn from_str(version: &str) -> Option<Version> {
        match u8::from_str_radix(version, 10) {
            Ok(n)  => Version::from_u8(n),
            Err(_) => None
        }
    }
    
    /// Convert a number representation of a version to a Version
    pub fn from_u8(version: u8) -> Option<Version> {
        match version {
            VERSION_1 => Some(Version::V1),
            VERSION_2 => Some(Version::V2),
            VERSION_3 => Some(Version::V3),
            VERSION_4 => Some(Version::V4),
            VERSION_5 => Some(Version::V5),
            _         => None
        }
    }
}