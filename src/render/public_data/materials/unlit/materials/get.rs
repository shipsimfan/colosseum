use crate::render::{UnlitMaterial, UnlitShader, public_data::materials::UnlitMaterials};
use alexandria::{Id, gpu::VulkanPipelineLayout};
use std::sync::Arc;

impl UnlitMaterials {
    /// Get the ID of the default unlit material
    pub fn default(&self) -> Id<UnlitMaterial> {
        self.default_material
    }

    /// Get a reference to the default unlit shader
    pub fn default_shader(&self) -> &Arc<UnlitShader> {
        &self.default_shader
    }

    /// Try to get a reference to an unlit material by its ID
    pub fn get(&self, id: Id<UnlitMaterial>) -> Option<&UnlitMaterial> {
        self.materials.get(id)
    }

    /// Try to get a mutable reference to an unlit material by its ID
    pub fn get_mut(&mut self, id: Id<UnlitMaterial>) -> Option<&mut UnlitMaterial> {
        self.materials.get_mut(id)
    }

    /// Get a reference to the unlit pipeline layout
    pub fn pipeline_layout(&self) -> &VulkanPipelineLayout {
        &self.pipeline_layout
    }
}
