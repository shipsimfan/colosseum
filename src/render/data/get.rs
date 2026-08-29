use crate::render::{
    AntiAliasingMode, CameraRenderData, LightingData, Material, Mesh, ObjectData, RenderData,
    RenderObjectRemoveConfirm, Skybox, data::DoubledRenderData,
};
use alexandria::{
    Id,
    gpu::{GpuAddress, VulkanDescriptorSet},
};
use std::vec::Drain;

impl RenderData {
    /// Returns a drain iterator over the confirmed removals in the render data
    pub fn confirmed_removals<'a>(&'a mut self) -> Drain<'a, RenderObjectRemoveConfirm> {
        self.confirmed_removals.drain(..)
    }

    /// Get the render scale for the current frame
    pub(in crate::render) fn render_scale(&self) -> f32 {
        self.render_scale
    }

    /// Get the gamma for the current frame
    pub(in crate::render) fn gamma(&self) -> f32 {
        self.gamma
    }

    /// Get the exposure for the current frame
    pub(in crate::render) fn exposure(&self) -> f32 {
        self.exposure
    }

    /// Get the contrast for the current frame
    pub(in crate::render) fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Get the saturation for the current frame
    pub(in crate::render) fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Get the anti-aliasing mode for the current frame
    pub(in crate::render) fn anti_aliasing(&self) -> AntiAliasingMode {
        self.anti_aliasing
    }

    /// Get the list of post processing descriptor sets for the current frame
    pub(in crate::render) fn post_process_descriptor_set(
        &self,
        index: usize,
    ) -> &VulkanDescriptorSet {
        &self.post_process_descriptor_sets[index]
    }

    /// Get a reference to the skybox that should be rendered in the current frame
    pub(in crate::render) fn skybox(&self) -> &Skybox {
        &self.skybox
    }

    /// Get the list of unlit opaque renderable objects in the render data
    pub(in crate::render) fn unlit_opaque_renderables(
        &self,
    ) -> impl Iterator<Item = (Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)> {
        self.doubled().unlit_opaque_renderables().iter().copied()
    }

    /// Get the list of lit opaque renderable objects in the render data
    pub(in crate::render) fn lit_opaque_renderables(
        &self,
    ) -> impl Iterator<Item = (Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)> {
        self.doubled().lit_opaque_renderables().iter().copied()
    }

    /// Get the camera data for the current frame
    pub(in crate::render) fn camera(&self) -> &CameraRenderData {
        self.doubled().camera()
    }

    /// Get a mutable reference to the camera data for the current frame
    pub fn camera_mut(&mut self) -> &mut CameraRenderData {
        self.doubled_mut().camera_mut()
    }

    /// Get the lighting data for the current frame
    pub(in crate::render) fn lighting(&self) -> &LightingData {
        self.doubled().lighting()
    }

    /// Get a mutable reference to the lighting daata for the current frame
    pub fn lighting_mut(&mut self) -> &mut LightingData {
        self.doubled_mut().lighting_mut()
    }

    /// Get a reference to the doubled render data that is currently being used for rendering
    fn doubled(&self) -> &DoubledRenderData {
        &self.doubled[self.current_doubled_index]
    }

    /// Get a mutable reference to the doubled render data that is currently being used for rendering
    pub(in crate::render::data) fn doubled_mut(&mut self) -> &mut DoubledRenderData {
        &mut self.doubled[self.current_doubled_index]
    }
}
