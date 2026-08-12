use crate::render::{
    CameraRenderData, Material, Mesh, RenderData, RenderObjectRemoveConfirm, Skybox,
    data::DoubledRenderData,
};
use alexandria::Id;
use std::vec::Drain;

impl RenderData {
    /// Returns a drain iterator over the confirmed removals in the render data
    pub fn confirmed_removals<'a>(&'a mut self) -> Drain<'a, RenderObjectRemoveConfirm> {
        self.confirmed_removals.drain(..)
    }

    /// Get a reference to the skybox that should be rendered in the current frame
    pub(in crate::render) fn skybox(&self) -> &Skybox {
        &self.skybox
    }

    /// Get the list of unlit opaque renderable objects in the render data
    pub(in crate::render) fn unlit_opaque_renderables(&self) -> &[(Id<Material>, Id<Mesh>)] {
        &self.unlit_opaque_renderables
    }

    /// Get the camera data for the current frame
    pub(in crate::render) fn camera(&self) -> &CameraRenderData {
        self.doubled().camera()
    }

    /// Get a reference to the doubled render data that is currently being used for rendering
    fn doubled(&self) -> &DoubledRenderData {
        &self.doubled[self.current_doubled_index]
    }
}
