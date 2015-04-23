use std::borrow::{ToOwned};

use forum;
use version::{Version};

pub const BASIC_DEVICE_NAME: &'static str = "Basic";
const MEDIA_SERVER_NAME:     &'static str = "MediaServer";
const MEDIA_RENDERER_NAME:   &'static str = "MediaRenderer";
const MANAGED_DEVICE_NAME:   &'static str = "ManageableDevice";
const SOLAR_BLIND_NAME:      &'static str = "SolarProtectionBlind";
const SECURITY_CAMERA_NAME:  &'static str = "DigitalSecurityCamera";
const HVAC_SYSTEM_NAME:      &'static str = "HVAC_System";
const BINARY_LIGHT_NAME:     &'static str = "BinaryLight";
const DIMMABLE_LIGHT_NAME:   &'static str = "DimmableLight";
const INTERNET_GATEWAY_NAME: &'static str = "InternetGatewayDevice";
const WIRELESS_AP_NAME:      &'static str = "WLANAccessPointDevice";
const PRINTER_NAME:          &'static str = "Printer";
const SCANNER_NAME:          &'static str = "Scanner";
const SENSOR_MANAGER_NAME:   &'static str = "SensorManagement";
const TELEPHONY_CLIENT_NAME: &'static str = "TelephonyClient";
const TELEPHONY_SERVER_NAME: &'static str = "TelephonyServer";

/// Device types included in the UPnP Forum layer of the UPnP architecture.
///
/// Unimplemented: MultiScreen, RemoteAccess, Remoting
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DeviceType {
    /// Device that provides basic information about itself.
    BasicDevice(Version),
    /// Device that provides an interface for accessing a MediaServer.
    MediaServer(Version),
    /// Device that provides an interface for accessing a MediaRenderer.
    MediaRenderer(Version),
    /// Device that provides an interface for maintenance.
    ManagedDevice(Version),
    /// Device that provides an interface for adjusting blinds.
    SolarBlind(Version),
    /// Device that provides an interface for accessing a security camera.
    SecurityCamera(Version),
    /// Device that provides an interface for controlling an HVAC system.
    HVACSystem(Version),
    /// Device that provides an interface for toggling a light.
    BinaryLight(Version),
    /// Device that provides an interface for toggling and dimming a light.
    DimmableLight(Version),
    /// Device that provides an interface for interfacing with a router.
    InternetGateway(Version),
    /// Device that provides an interface for interfacing with a wireless access point.
    WirelessAP(Version),
    /// Device that provides an interface for print services.
    Printer(Version),
    /// Device that provides an interface for scanning services.
    Scanner(Version),
    /// Device that provides an interface for accessing sensors and actuators.
    SensorManager(Version),
    /// Device that provides an interface for controlling a telephony client.
    TelephonyClient(Version),
    /// Device that provides an interface for controlling a telephony server.
    TelephonyServer(Version),
    /// Device that has not been implemented.
    Unimplemented(String, Version)
    ///// Device not included in the UPnP Forum layer.
    //Vendor(VendorDeviceType)
}

impl DeviceType {
    /// Create a new DeviceType from the given values.
    pub fn new(schema: &str, dev_type: &str, version: Version) -> DeviceType {
        match schema {
            forum::UPNP_SCHEMA_VALUE => match_vendor_name(dev_type, version),
            _ => panic!("VendorDeviceType Unimplemented")//DeviceType::Vendor(VendorDeviceType::new(schema, type, version))
        }
    }
    
    pub fn version(&self) -> Version {
        match *self {
            DeviceType::BasicDevice(n)      => n,
            DeviceType::MediaServer(n)      => n,
            DeviceType::MediaRenderer(n)    => n,
            DeviceType::ManagedDevice(n)    => n,
            DeviceType::SolarBlind(n)       => n,
            DeviceType::SecurityCamera(n)   => n,
            DeviceType::HVACSystem(n)       => n,
            DeviceType::BinaryLight(n)      => n,
            DeviceType::DimmableLight(n)    => n,
            DeviceType::InternetGateway(n)  => n,
            DeviceType::WirelessAP(n)       => n,
            DeviceType::Printer(n)          => n,
            DeviceType::Scanner(n)          => n,
            DeviceType::SensorManager(n)    => n,
            DeviceType::TelephonyClient(n)  => n,
            DeviceType::TelephonyServer(n)  => n,
            DeviceType::Unimplemented(_, n) => n
        }
    }
}

/// Match the type to a vendor name.
///
/// Returns the appropriate DeviceType.
fn match_vendor_name(dev_type: &str, version: Version) -> DeviceType {
    match dev_type {
        BASIC_DEVICE_NAME     => DeviceType::BasicDevice(version),
        MEDIA_SERVER_NAME     => DeviceType::MediaServer(version),
        MEDIA_RENDERER_NAME   => DeviceType::MediaRenderer(version),
        MANAGED_DEVICE_NAME   => DeviceType::ManagedDevice(version),
        SOLAR_BLIND_NAME      => DeviceType::SolarBlind(version),
        SECURITY_CAMERA_NAME  => DeviceType::SecurityCamera(version),
        HVAC_SYSTEM_NAME      => DeviceType::HVACSystem(version),
        BINARY_LIGHT_NAME     => DeviceType::BinaryLight(version),
        DIMMABLE_LIGHT_NAME   => DeviceType::DimmableLight(version),
        INTERNET_GATEWAY_NAME => DeviceType::InternetGateway(version),
        WIRELESS_AP_NAME      => DeviceType::WirelessAP(version),
        PRINTER_NAME          => DeviceType::Printer(version),
        SCANNER_NAME          => DeviceType::Scanner(version),
        SENSOR_MANAGER_NAME   => DeviceType::SensorManager(version),
        TELEPHONY_CLIENT_NAME => DeviceType::TelephonyClient(version),
        TELEPHONY_SERVER_NAME => DeviceType::TelephonyServer(version),
        _ => DeviceType::Unimplemented(dev_type.to_owned(), version)
    }
}

#[cfg(test)]
mod tests {
    use forum;
    use super::{DeviceType};
    use version::{Version};

    #[test]
    fn positive_upnp_schema() {
        let device_type = DeviceType::new("schemas-upnp-org", "Basic", Version::V1);
        
        assert_eq!(device_type, DeviceType::BasicDevice(Version::V1));
    }
    
    #[test]
    fn positive_unimplemented_device() {
        let device_type = DeviceType::new("schemas-upnp-org", "Blargenfargen", Version::V1);
        
        assert_eq!(device_type, DeviceType::Unimplemented("Blargenfargen".to_string(), Version::V1));
    }
}