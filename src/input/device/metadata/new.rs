use crate::input::InputDeviceMetadata;

impl InputDeviceMetadata {
    /// Create a new [`InputDeviceMetadata`]
    pub(crate) fn new(
        vendor_id: u32,
        product_id: u32,
        version_number: u32,
        usage_page: u16,
        usage: u16,
    ) -> Self {
        InputDeviceMetadata {
            vendor_id,
            product_id,
            version_number,
            usage_page,
            usage,
        }
    }
}
