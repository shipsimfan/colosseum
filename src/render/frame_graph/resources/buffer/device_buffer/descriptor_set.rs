/// A descriptor set that a device buffer is bound to
pub(in crate::render) struct DeviceBufferDescriptorSet {
    /// The index of the descriptor set to bind to
    pub descriptor_set: usize,

    /// The location in `descriptor_set` to bind the buffer to
    pub binding: u32,
}

impl From<(u32, usize)> for DeviceBufferDescriptorSet {
    fn from((binding, descriptor_set): (u32, usize)) -> Self {
        DeviceBufferDescriptorSet {
            descriptor_set,
            binding,
        }
    }
}
