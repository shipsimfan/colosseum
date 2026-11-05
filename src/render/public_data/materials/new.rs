use crate::{
    Result,
    render::{
        DescriptorSetLayouts, GraphicsDevice, Materials,
        public_data::materials::{LitForwardMaterials, UnlitMaterials},
    },
};
use std::sync::Arc;

impl Materials {
    /// Create a new set of [`Materials`]
    pub(in crate::render::public_data) fn new(
        descriptor_set_layouts: &DescriptorSetLayouts,
        device: &Arc<GraphicsDevice>,
    ) -> Result<Materials> {
        let unlit = UnlitMaterials::new(descriptor_set_layouts, device)?;
        let lit_forward = LitForwardMaterials::new(descriptor_set_layouts, device)?;

        Ok(Materials {
            unlit,
            lit_forward,
            device: device.clone(),
        })
    }
}
