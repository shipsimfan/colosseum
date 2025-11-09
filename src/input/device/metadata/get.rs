use crate::input::InputDeviceMetadata;

impl InputDeviceMetadata {
    /// Get the ID identifying the vendor of the product
    pub fn vendor_id(&self) -> u32 {
        self.vendor_id
    }

    /// Get the vendor assigned ID identifying this product
    pub fn product_id(&self) -> u32 {
        self.product_id
    }

    /// Get the version of the input device
    pub fn version_number(&self) -> u32 {
        self.version_number
    }

    /// Get the usage page the device is in
    pub fn usage_page(&self) -> u16 {
        self.usage_page
    }

    /// Get the usage of the device
    pub fn usage(&self) -> u16 {
        self.usage
    }
}
