use crate::{
    Result,
    render::{CameraDsl, DescriptorSetLayouts, GraphicsDevice},
};

impl CameraDsl {
    /// Create a new set of [`CameraDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<CameraDsl>("Camera Descriptor Set Layout", device)
    }
}
