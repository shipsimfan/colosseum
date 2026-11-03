use crate::render::{Materials, UnlitMaterial, UnlitShader};
use alexandria::{Id, gpu::VulkanPipelineLayout};
use std::sync::Arc;

impl Materials {
    /// Get the ID of the default unlit material
    pub fn default_unlit_material(&self) -> Id<UnlitMaterial> {
        self.default_unlit_material
    }

    /// Get a reference to the default unlit shader
    pub fn default_unlit_shader(&self) -> &Arc<UnlitShader> {
        &self.default_unlit_shader
    }

    /// Try to get a reference to an unlit material by its ID
    pub fn try_get_unlit_material(&self, id: Id<UnlitMaterial>) -> Option<&UnlitMaterial> {
        self.unlit_materials.get(id)
    }

    /// Get a reference to an unlit material by its ID
    pub fn unlit_material(&self, id: Id<UnlitMaterial>) -> &UnlitMaterial {
        &self.unlit_materials[id]
    }

    /// Try to get a mutable reference to an unlit material by its ID
    pub fn try_get_unlit_material_mut(
        &mut self,
        id: Id<UnlitMaterial>,
    ) -> Option<&mut UnlitMaterial> {
        self.unlit_materials.get_mut(id)
    }

    /// Get a mutable reference to an unlit material by its ID
    pub fn unlit_material_mut(&mut self, id: Id<UnlitMaterial>) -> &mut UnlitMaterial {
        &mut self.unlit_materials[id]
    }

    /// Get a reference to the unlit pipeline layout
    pub(in crate::render) fn unlit_pipeline_layout(&self) -> &VulkanPipelineLayout {
        &self.unlit_pipeline_layout
    }
}
