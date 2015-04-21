use forum::device::basic_device::{BasicDeviceQuery};
use forum::device::binary_light::{BinaryLightQuery};
use forum::device::dimmable_light::{DimmableLightQuery};
use forum::device::generic_device::{GenericDeviceQuery};
use forum::device::hvac_system::{HVACSystemQuery};
use forum::device::internet_gateway::{InternetGatewayQuery};
use forum::device::managed_device::{ManagedDeviceQuery};
use forum::device::media_renderer::{MediaRendererQuery};
use forum::device::media_server::{MediaServerQuery};
use forum::device::printer::{PrinterQuery};
use forum::device::scanner::{ScannerQuery};
use forum::device::security_camera::{SecurityCameraQuery};
use forum::device::sensor_manager::{SensorManagerQuery};
use forum::device::solar_blind::{SolarBlindQuery};
use forum::device::telephony_client::{TelephonyClientQuery};
use forum::device::telephony_server::{TelephonyServerQuery};
use forum::device::wireless_ap::{WirelessAPQuery};
use forum::device::{DeviceType};
use forum::query::{GenericQuery};

/// Enumerates all device types as well as their corresponding query objects.
///
/// All device query objects can be thought of as typed GenericQuery objects.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DeviceQuery {
    /// Device that provides basic information about itself.
    BasicDevice(BasicDeviceQuery),
    /// Device that provides an interface for accessing a MediaServer.
    MediaServer(MediaServerQuery),
    /// Device that provides an interface for accessing a MediaRenderer.
    MediaRenderer(MediaRendererQuery),
    /// Device that provides an interface for maintenance.
    ManagedDevice(ManagedDeviceQuery),
    /// Device that provides an interface for adjusting blinds.
    SolarBlind(SolarBlindQuery),
    /// Device that provides an interface for accessing a security camera.
    SecurityCamera(SecurityCameraQuery),
    /// Device that provides an interface for controlling an HVAC system.
    HVACSystem(HVACSystemQuery),
    /// Device that provides an interface for toggling a light.
    BinaryLight(BinaryLightQuery),
    /// Device that provides an interface for toggling and dimming a light.
    DimmableLight(DimmableLightQuery),
    /// Device that provides an interface for interfacing with a router.
    InternetGateway(InternetGatewayQuery),
    /// Device that provides an interface for interfacing with a wireless access point.
    WirelessAP(WirelessAPQuery),
    /// Device that provides an interface for basic print services.
    Printer(PrinterQuery),
    /// Device that provides an interface for scanning services.
    Scanner(ScannerQuery),
    /// Device that provides an interface for accessing sensors and actuators.
    SensorManager(SensorManagerQuery),
    /// Device that provides an interface for controlling a telephony client.
    TelephonyClient(TelephonyClientQuery),
    /// Device that provides an interface for controlling a telephony server.
    TelephonyServer(TelephonyServerQuery),
    /// Device that has not been implemented.
    Unimplemented(GenericDeviceQuery),
    ///// Device not included in the UPnP Forum layer.
    //Vendor(VendorDeviceQuery)
}

impl DeviceQuery {
    /// Create a new DeviceQuery from the given query and device type.
    pub fn new(query: GenericQuery, dev_type: DeviceType) -> DeviceQuery {
        match_device_query(query, dev_type)
    }
    
    pub fn uuid(&self) -> &[u8] {
        &b"Unimplemented"[..]
    }
}

/// Match and construct the appropriate query for the given device type.
///
/// Returns the matched DeviceQuery.
fn match_device_query(query: GenericQuery, dev_type: DeviceType) -> DeviceQuery {
    match dev_type {
        DeviceType::BasicDevice(_) => {
            DeviceQuery::BasicDevice(BasicDeviceQuery::new(query, dev_type))
        },
        DeviceType::MediaServer(_) => {
            DeviceQuery::MediaServer(MediaServerQuery::new(query, dev_type))
        },
        DeviceType::MediaRenderer(_) => {
            DeviceQuery::MediaRenderer(MediaRendererQuery::new(query, dev_type))
        },
        DeviceType::ManagedDevice(_) => {
            DeviceQuery::ManagedDevice(ManagedDeviceQuery::new(query, dev_type))
        },
        DeviceType::SolarBlind(_) => {
            DeviceQuery::SolarBlind(SolarBlindQuery::new(query, dev_type))
        },
        DeviceType::SecurityCamera(_) => {
            DeviceQuery::SecurityCamera(SecurityCameraQuery::new(query, dev_type))
        },
        DeviceType::HVACSystem(_) => {
            DeviceQuery::HVACSystem(HVACSystemQuery::new(query, dev_type))
        },
        DeviceType::BinaryLight(_) => {
            DeviceQuery::BinaryLight(BinaryLightQuery::new(query, dev_type))
        },
        DeviceType::DimmableLight(_) => {
            DeviceQuery::DimmableLight(DimmableLightQuery::new(query, dev_type))
        },
        DeviceType::InternetGateway(_) => {
            DeviceQuery::InternetGateway(InternetGatewayQuery::new(query, dev_type))
        },
        DeviceType::WirelessAP(_) => {
            DeviceQuery::WirelessAP(WirelessAPQuery::new(query, dev_type))
        },
        DeviceType::Printer(_) => {
            DeviceQuery::Printer(PrinterQuery::new(query, dev_type))
        },
        DeviceType::Scanner(_) => {
            DeviceQuery::Scanner(ScannerQuery::new(query, dev_type))
        },
        DeviceType::SensorManager(_) => {
            DeviceQuery::SensorManager(SensorManagerQuery::new(query, dev_type))
        },
        DeviceType::TelephonyClient(_) => {
            DeviceQuery::TelephonyClient(TelephonyClientQuery::new(query, dev_type))
        },
        DeviceType::TelephonyServer(_) => {
            DeviceQuery::TelephonyServer(TelephonyServerQuery::new(query, dev_type))
        },
        DeviceType::Unimplemented(_, _) => {
            DeviceQuery::Unimplemented(GenericDeviceQuery::new(query, dev_type))
        }
    }
}