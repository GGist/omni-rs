
mod query;
mod typing;

pub mod basic_device;
pub mod media_server;
pub mod media_renderer;
pub mod managed_device;
pub mod solar_blind;
pub mod security_camera;
pub mod hvac_system;
pub mod binary_light;
pub mod dimmable_light;
pub mod internet_gateway;
pub mod wireless_ap;
pub mod printer;
pub mod scanner;
pub mod sensor_manager;
pub mod telephony_client;
pub mod telephony_server;
pub mod generic_device;

pub use forum::device::query::DeviceQuery;
pub use forum::device::typing::DeviceType;
//
/*
enum Device {
    /// Device that provides basic information about itself.
    BasicDevice(BasicDevice),
    /// Device that provides an interface for accessing a MediaServer.
    MediaServer(MediaServer),
    /// Device that provides an interface for accessing a MediaRenderer.
    MediaRenderer(MediaRenderer),
    /// Device that provides an interface for maintenance.
    ManagedDevice(ManagedDevice),
    /// Device that provides an interface for adjusting blinds.
    SolarBlind(SolarBlind),
    /// Device that provides an interface for accessing a security camera.
    SecurityCamera(SecurityCamera),
    /// Device that provides an interface for controlling an HVAC system.
    HVACSystem(HVACSystem),
    /// Device that provides an interface for toggling a light.
    BinaryLight(BinaryLight),
    /// Device that provides an interface for toggling and dimming a light.
    DimmableLight(DimmableLight),
    /// Device that provides an interface for interfacing with a router.
    InternetGateway(InternetGateway),
    /// Device that provides an interface for interfacing with a wireless access point.
    WirelessAP(WirelessAP),
    /// Device that provides an interface for basic print services.
    BasicPrinter(BasicPrinter),
    /// Device that provides an interface for advanced print services.
    AdvancedPrinter(AdvancedPrinter),
    /// Device that provides an interface for scanning services.
    Scanner(Scanner),
    /// Device that provides an interface for accessing sensors and actuators.
    Sensor(Sensor),
    /// Device that provides an interface for controlling a telephony client.
    TelephonyClient(TelephonyClient),
    /// Device that provides an interface for controlling a telephony server.
    TelephonyServer(TelephonyServer),
    /// Device that has not been implemented.
    Unimplemented(GenericDevice),
    /// Device not included in the UPnP Forum layer.
    //Vendor(VendorDevice)
}*/

