mod get;
mod new;

/// Information which can identify the specific kind of device
pub struct InputDeviceMetadata {
    /// An ID identifying the vendor of the device
    vendor_id: u32,

    /// An ID identifying the product from the vendor
    product_id: u32,

    /// A number indicating what version of the product this device is
    version_number: u32,

    /// The usage page this device is in
    usage_page: u16,

    /// The specific usage of this device
    usage: u16,
}
