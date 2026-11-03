use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, RenderableDsl},
};

impl RenderableDsl {
    /// Create a new set of [`RenderableDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<RenderableDsl>("Renderable Descriptor Set Layout", device)
    }
}
