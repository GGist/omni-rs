use version::{Version};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ServiceType {
    PlaceHolder
}

impl ServiceType {
    /// Create a new ServiceType from the given values.
    pub fn new(_: &str, _: &str, _: Version) -> ServiceType {
        ServiceType::PlaceHolder
    }
}