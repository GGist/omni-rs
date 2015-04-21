/// Type of notification received from some interface.
/// Data in each variant corresponds to Unique Service Name (USN) inforamtion.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum SearchTarget {
    All,
    Root,
    Unique(UUID),
    StandardDevice(
    /// Signifies either a nested device or a root device.
    Device(UUID),
    /// Signifies a root device.
    RootDevice(UUID),
    /// Signifies a nested device.
    NestedDevice(UUID, SchemaType, DeviceType, Version),
    /// Signifies a nested service.
    NestedService(UUID, SchemaType, ServiceType, Version)
}

/// SUPPORT MULTICAST AND UNICAST
/// http://stackoverflow.com/questions/28870802/why-cant-i-get-upnp-unicast-m-search-to-work-instead-of-multicast-m-search
/// SEARCHING....