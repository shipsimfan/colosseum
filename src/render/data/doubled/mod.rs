use crate::render::CameraRenderData;

mod get;
mod new;

/// The render data that exists in two copies for each frame, so that one copy can be used for
/// rendering while the other is being updated
pub(in crate::render::data) struct DoubledRenderData {
    /// The camera data for the current frame
    camera: CameraRenderData,
}
