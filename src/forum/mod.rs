//! Implements the UPnP Device Architecture and UPnP Forum layers of the UPnP
//! standard.
//! 
//! This module deals with device and service identification and
//! interaction for devices and services that have been standardized by the UPnP
//! working committee.

use std::{str};

use {SimpleError, SimpleResult};
use ssdp::{FieldPair};
use forum::device::{DeviceType};
use forum::service::{ServiceType};
use version::{Version};

mod query;

pub mod device;
pub mod service;

pub use forum::query::GenericQuery;

const URN_TARGET_SEPARATOR: char = ':';

const UPNP_SCHEMA_VALUE: &'static str = "schemas-upnp-org";
const UPNP_ROOT_VALUE:   &'static str = "rootdevice";
const URN_DEVICE_VALUE:  &'static str = "device";
const URN_SERVICE_VALUE: &'static str = "service";

/// Represents a class of devices or services.
///
/// Used to denote all root devices, specific devices, all devices of the form
/// (schema, type, version), or all services of the form (schema, type, version).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TargetType {
    Root,
    UUID,
    Device(DeviceType),
    Service(ServiceType)
}

impl TargetType {
    /// Create a new target type from the given target value.
    fn new(target: &FieldPair) -> SimpleResult<TargetType> {
        match *target {
            FieldPair::UPnP(ref n) => validate_upnp(&n[..]),
            FieldPair::UUID(ref n) => validate_uuid(&n[..]),
            FieldPair::URN(ref n)  => parse_urn(&n[..]),
            FieldPair::Unknown(..) => {
                Err(SimpleError::new("Target Unknown Field Key Is Invalid As A Target Type"))
            }
        }
    }
}

/// Validate the value of a upnp target.
///
/// Returns the corresponding TargetType or an error.
fn validate_upnp(value: &[u8]) -> SimpleResult<TargetType> {
    if value == UPNP_ROOT_VALUE.as_bytes() {
        Ok(TargetType::Root)
    } else {
        Err(SimpleError::new("Target UPnP Field Value Is Invalid"))
    }
}

/// Validate the value of a uuid target.
///
/// Returns the corresponding TargetType or an error.
fn validate_uuid(value: &[u8]) -> SimpleResult<TargetType> {
    // Only guarantee we can make for UPnP 1.0 UUID values is that they are
    // valid strings, so check that our value is valid utf-8.
    match str::from_utf8(value) {
        Ok(_)  => Ok(TargetType::UUID),
        Err(_) => Err(SimpleError::new("Target UUID Value Is Invalid UTF-8"))
    }
}

/// Parse the value of a urn target.
///
/// Returns the corresponding TargetType or an error.
fn parse_urn(value: &[u8]) -> SimpleResult<TargetType> {
    let target_str = match str::from_utf8(value) {
        Ok(n)  => n,
        Err(_) => return Err(SimpleError::new("Target URN Value Is Invalid UTF-8"))
    };
    let mut target_iter = target_str.split(URN_TARGET_SEPARATOR);
    
    let target_schema = match target_iter.next() {
        Some(n) => n,
        None    => return Err(SimpleError::new("Target URN Value Is Missing Schema"))
    };
    
    let target_class = match target_iter.next() {
        Some(n) => n,
        None    => return Err(SimpleError::new("Target URN Value Is Missing Class"))
    };
    
    let target_type = match target_iter.next() {
        Some(n) => n,
        None    => return Err(SimpleError::new("Target URN Value Is Missing Type"))
    };
    
    let target_version = match target_iter.next() {
        Some(n) => {
            if let Some(n) = Version::from_str(n) {
                n
            } else {
                return Err(SimpleError::new("Target URN Value Has Invalid Version"))
            }
        },
        None => return Err(SimpleError::new("Target URN Value Is Missing Version"))
    };
    
    // Make sure there were no more peices of the split string
    if target_iter.next().is_some() {
        return Err(SimpleError::new("Target URN Value Has Invalid Format"))
    }
    
    match target_class {
        URN_DEVICE_VALUE  => {
            let device = DeviceType::new(target_schema, target_type, target_version);
            Ok(TargetType::Device(device))
        },
        URN_SERVICE_VALUE => {
            let service = ServiceType::new(target_schema, target_type, target_version);
            Ok(TargetType::Service(service))
        },
        _ => Err(SimpleError::new("Target URN Value Has Invalid Target Class (Device/Service)"))
    }
}

#[cfg(test)]
mod tests {
    use forum::device::{DeviceType};
    use ssdp::{FieldPair};
    use version::{Version};
    
    use super::{TargetType};
    
    #[test]
    fn positive_root() {
        let root_pair = FieldPair::UPnP(b"rootdevice".to_vec());
        
        assert_eq!(TargetType::new(&root_pair).unwrap(), TargetType::Root);
    }
    
    #[test]
    fn positive_uuid() {
        let uuid_pair = FieldPair::UUID(b"valid_utf8_is_a_valid_uuid_in_upnp_10".to_vec());
        
        assert_eq!(TargetType::new(&uuid_pair).unwrap(), TargetType::UUID);
    }
    
    #[test]
    fn positive_device() {
        let urn_pair = FieldPair::URN(b"schemas-upnp-org:device:Basic:1".to_vec());
        
        assert_eq!(TargetType::new(&urn_pair).unwrap(), TargetType::Device(DeviceType::BasicDevice(Version::V1)));
    }
    
    #[test]
    fn positive_service() {
        let urn_pair = FieldPair::URN(b"schemas-upnp-org:device:Basic:1".to_vec());
        
        assert_eq!(TargetType::new(&urn_pair).unwrap(), TargetType::Device(DeviceType::BasicDevice(Version::V1)));
    }
    
    #[test]
    #[should_panic]
    fn negative_root() {
        let root_pair = FieldPair::UPnP(b"root_device".to_vec());
        
        TargetType::new(&root_pair).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_uuid() {
        let uuid_pair = FieldPair::UUID(b"woops_invalid_utf8_\x80".to_vec());
        
        TargetType::new(&uuid_pair).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_missing_urn_schema() {
        let urn_pair = FieldPair::URN(b":device:Basic:1".to_vec());
        
        TargetType::new(&urn_pair).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_bad_urn_class() {
        let urn_pair = FieldPair::URN(b"schemas-upnp-org:blarg:Basic:1".to_vec());
        
        TargetType::new(&urn_pair).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_bad_urn_version() {
        let urn_pair = FieldPair::URN(b"schemas-upnp-org:device:Basic:one".to_vec());
        
        TargetType::new(&urn_pair).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn negative_extra_fields() {
        let urn_pair = FieldPair::URN(b"schemas-upnp-org:device:Basic:1:extra".to_vec());
        
        TargetType::new(&urn_pair).unwrap();
    }
}