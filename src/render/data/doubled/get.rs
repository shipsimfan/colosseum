use crate::render::{CameraRenderData, Material, Mesh, ObjectData, data::DoubledRenderData};
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

    /// Get the list of unlit opaque renderable objects in the doubled render data
    pub fn unlit_opaque_renderables(&self) -> &[(Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)] {
        &self.unlit_opaque_renderables
    }
}
