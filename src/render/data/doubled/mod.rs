use crate::render::{CameraRenderData, ObjectData, data::RenderableList};

mod get;
mod new;

/// The render data that exists in two copies for each frame, so that one copy can be used for
/// rendering while the other is being updated
pub(in crate::render::data) struct DoubledRenderData {
    /// The camera data for the current frame
    camera: CameraRenderData,

    /// The set of unlit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not require any lighting
    /// calculations or transparency
    unlit_opaque_renderables: RenderableList<ObjectData>,
}
