use std::sync::Arc;

use crate::render::{
    LitForwardMaterial, LitForwardShader, public_data::materials::LitForwardMaterials,
};
use alexandria::{Id, gpu::VulkanPipelineLayout};

impl LitForwardMaterials {
    /// Get the ID of the default lit forward material
    pub fn default(&self) -> Id<LitForwardMaterial> {
        self.default_material
    }

    /// Get a reference to the default lit forward shader
    pub fn default_shader(&self) -> &Arc<LitForwardShader> {
        &self.default_shader
    }

    /// Try to get a reference to a lit forward material by its ID
    pub fn get(&self, id: Id<LitForwardMaterial>) -> Option<&LitForwardMaterial> {
        self.materials.get(id)
    }

    /// Try to get a mutable reference to a lit forward material by its ID
    pub fn get_mut(&mut self, id: Id<LitForwardMaterial>) -> Option<&mut LitForwardMaterial> {
        self.materials.get_mut(id)
    }

    /// Get a reference to the lit pipeline layout for the forward render pass
    pub fn pipeline_layout(&self) -> &VulkanPipelineLayout {
        &self.pipeline_layout
    }
}
