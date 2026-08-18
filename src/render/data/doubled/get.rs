use crate::render::{
    CameraRenderData, ObjectData,
    data::{DoubledRenderData, RenderableList},
};

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
    pub fn unlit_opaque_renderables(&self) -> &RenderableList<ObjectData> {
        &self.unlit_opaque_renderables
    }

    /// Get a mutable reference to the list of unlit opaque renderable objects in the doubled
    /// render data
    pub fn unlit_opaque_renderables_mut(&mut self) -> &mut RenderableList<ObjectData> {
        &mut self.unlit_opaque_renderables
    }
}
