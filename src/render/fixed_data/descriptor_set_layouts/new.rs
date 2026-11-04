use crate::{
    Result,
    render::{
        CameraDsl, DescriptorSetLayouts, DirectionalLightDsl, GraphicsDevice, PointLightDsl,
        PostProcessDsl, RenderableDsl, SpotLightDsl,
    },
};

impl DescriptorSetLayouts {
    /// Create a new set of [`DescriptorSetLayouts`]
    pub fn new(device: &GraphicsDevice) -> Result<DescriptorSetLayouts> {
        let mut layouts = DescriptorSetLayouts {
            layouts: Vec::new(),
        };

        PostProcessDsl::new(&mut layouts, device)?;
        CameraDsl::new(&mut layouts, device)?;
        RenderableDsl::new(&mut layouts, device)?;
        DirectionalLightDsl::new(&mut layouts, device)?;
        PointLightDsl::new(&mut layouts, device)?;
        SpotLightDsl::new(&mut layouts, device)?;

        Ok(layouts)
    }
}
