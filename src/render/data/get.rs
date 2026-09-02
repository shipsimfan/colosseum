use crate::render::{
    AntiAliasingMode, LightingData, LocalDataBuffer, ObjectData, RenderCamera, RenderData,
    RenderObjectRemoveConfirm, Renderable, Skybox,
};
use alexandria::gpu::VulkanFence;
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

    /// Get a reference to the skybox that should be rendered in the current frame
    pub(in crate::render) fn skybox(&self) -> &Skybox {
        &self.skybox
    }

    /// Get the list of unlit opaque renderable objects in the render data
    pub(in crate::render) fn unlit_opaque_renderables(&self) -> impl Iterator<Item = Renderable> {
        self.unlit_opaque_renderables.iter().copied()
    }

    /// Get the list of lit opaque renderable objects in the render data
    pub(in crate::render) fn lit_opaque_renderables(&self) -> impl Iterator<Item = Renderable> {
        self.lit_opaque_renderables.iter().copied()
    }

    /// Get the lighting data for the current frame
    pub(in crate::render) fn lighting(&self) -> &LightingData {
        &self.lighting
    }

    /// Get a reference to the local camera data buffer
    pub(in crate::render) fn camera(&self) -> &LocalDataBuffer<RenderCamera> {
        &self.camera
    }

    /// Get a reference to the renderables data buffer
    pub(in crate::render) fn renderables(&self) -> &LocalDataBuffer<ObjectData> {
        &self.renderable_buffer
    }

    /// Get the fence used to synchronize copy operations for the current frame
    pub(in crate::render) fn copy_fence(&mut self) -> &mut VulkanFence {
        self.copy_commands_sent = true;
        &mut self.copy_fence
    }

    /// Get a mutable reference to the lighting daata for the current frame
    pub fn lighting_mut(&mut self) -> &mut LightingData {
        &mut self.lighting
    }
}
