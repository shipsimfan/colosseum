use crate::{
    Result,
    render::{CameraDsl, DescriptorSetLayouts, GraphicsDevice, PostProcessDsl},
};

impl DescriptorSetLayouts {
    /// Create a new set of [`DescriptorSetLayouts`]
    pub fn new(device: &GraphicsDevice) -> Result<DescriptorSetLayouts> {
        let mut layouts = DescriptorSetLayouts {
            layouts: Vec::new(),
        };

        PostProcessDsl::new(&mut layouts, device)?;
        CameraDsl::new(&mut layouts, device)?;

        Ok(layouts)
    }
}
