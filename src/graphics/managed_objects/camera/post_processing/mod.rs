use crate::{
    graphics::PostProcessingShader,
    util::{Arena, Handle},
};
use render_scale_objects::RenderScaleObjects;

mod render_scale_objects;

mod bind;
mod clear;
mod new;
mod resize;

/// A handle to a provided post-process stage
pub type PostProcessHandle = Handle<PostProcessingShader>;

/// The camera related objects used during post processing
pub struct CameraPostProcessing {
    /// The provided post-process stages
    pub(in crate::graphics) provided_post_processing: Arena<PostProcessingShader>,

    /// The objects tied to the render scale and swapchain size
    pub(in crate::graphics) render_scale_objects: RenderScaleObjects,
}
