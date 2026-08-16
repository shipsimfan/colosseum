use crate::render::{
    CameraRenderData,
    data::{DoubledRenderData, RenderableList},
};
use alexandria::math::Matrix4x4f;

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
    pub fn unlit_opaque_renderables(&self) -> &RenderableList<Matrix4x4f> {
        &self.unlit_opaque_renderables
    }

    /// Get a mutable reference to the list of unlit opaque renderable objects in the doubled
    /// render data
    pub fn unlit_opaque_renderables_mut(&mut self) -> &mut RenderableList<Matrix4x4f> {
        &mut self.unlit_opaque_renderables
    }
}
