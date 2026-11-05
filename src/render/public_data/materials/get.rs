use crate::render::{LitForwardMaterial, LitForwardShader, Materials, UnlitMaterial, UnlitShader};
use alexandria::{Id, gpu::VulkanPipelineLayout};
use std::sync::Arc;

impl Materials {
    /// Get the ID of the default unlit material
    pub fn default_unlit(&self) -> Id<UnlitMaterial> {
        self.unlit.default()
    }

    /// Get a reference to the default unlit shader
    pub fn default_unlit_shader(&self) -> &Arc<UnlitShader> {
        self.unlit.default_shader()
    }

    /// Try to get a reference to an unlit material by its ID
    pub fn try_get_unlit_material(&self, id: Id<UnlitMaterial>) -> Option<&UnlitMaterial> {
        self.unlit.get(id)
    }

    /// Get a reference to an unlit material by its ID
    pub fn unlit_material(&self, id: Id<UnlitMaterial>) -> &UnlitMaterial {
        self.try_get_unlit_material(id)
            .expect("unlit material not found")
    }

    /// Try to get a mutable reference to an unlit material by its ID
    pub fn try_get_unlit_material_mut(
        &mut self,
        id: Id<UnlitMaterial>,
    ) -> Option<&mut UnlitMaterial> {
        self.unlit.get_mut(id)
    }

    /// Get a mutable reference to an unlit material by its ID
    pub fn unlit_material_mut(&mut self, id: Id<UnlitMaterial>) -> &mut UnlitMaterial {
        self.try_get_unlit_material_mut(id)
            .expect("unlit material not found")
    }

    /// Get the ID of the default lit forward material
    pub fn default_lit_forward(&self) -> Id<LitForwardMaterial> {
        self.lit_forward.default()
    }

    /// Get a reference to the default lit forward shader
    pub fn default_lit_forward_shader(&self) -> &Arc<LitForwardShader> {
        self.lit_forward.default_shader()
    }

    /// Try to get a reference to a lit forward material by its ID
    pub fn try_get_lit_forward_material(
        &self,
        id: Id<LitForwardMaterial>,
    ) -> Option<&LitForwardMaterial> {
        self.lit_forward.get(id)
    }

    /// Get a reference to a lit forward material by its ID
    pub fn lit_forward_material(&self, id: Id<LitForwardMaterial>) -> &LitForwardMaterial {
        self.try_get_lit_forward_material(id)
            .expect("lit forward material not found")
    }

    /// Try to get a mutable reference to a lit forward material by its ID
    pub fn try_get_lit_forward_material_mut(
        &mut self,
        id: Id<LitForwardMaterial>,
    ) -> Option<&mut LitForwardMaterial> {
        self.lit_forward.get_mut(id)
    }

    /// Get a mutable reference to a lit forward material by its ID
    pub fn lit_forward_material_mut(
        &mut self,
        id: Id<LitForwardMaterial>,
    ) -> &mut LitForwardMaterial {
        self.try_get_lit_forward_material_mut(id)
            .expect("lit forward material not found")
    }

    /// Get a reference to the unlit pipeline layout
    pub(in crate::render) fn unlit_pipeline_layout(&self) -> &VulkanPipelineLayout {
        self.unlit.pipeline_layout()
    }

    /// Get a reference to the lit forward pipeline layout
    pub(in crate::render) fn lit_forward_pipeline_layout(&self) -> &VulkanPipelineLayout {
        self.lit_forward.pipeline_layout()
    }
}
