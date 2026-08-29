use crate::render::{
    CameraRenderData, LightingData, Material, Mesh, ObjectData, data::DoubledRenderData,
};
use alexandria::{Id, gpu::GpuAddress};

impl DoubledRenderData {
    /// Get the camera data for the current frame
    pub fn camera(&self) -> &CameraRenderData {
        &self.camera
    }

    /// Get a mutable reference to the camera data for the current frame
    pub fn camera_mut(&mut self) -> &mut CameraRenderData {
        &mut self.camera
    }

    /// Get the lighting data for the current frame
    pub fn lighting(&self) -> &LightingData {
        &self.lighting
    }

    /// Get a mutable reference to the lighting data for the current frame
    pub fn lighting_mut(&mut self) -> &mut LightingData {
        &mut self.lighting
    }

    /// Get the list of unlit opaque renderable objects in the doubled render data
    pub fn unlit_opaque_renderables(&self) -> &[(Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)] {
        &self.unlit_opaque_renderables
    }

    /// Get the list of lit opaque renderable objects in the doubled render data
    pub fn lit_opaque_renderables(&self) -> &[(Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)] {
        &self.lit_opaque_renderables
    }
}
