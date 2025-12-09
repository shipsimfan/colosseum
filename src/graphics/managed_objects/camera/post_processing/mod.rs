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
mod run;
/// A handle to a provided post-process stage
pub type PostProcessHandle = Handle<PostProcessingShader>;

/// The camera related objects used during post processing
pub struct CameraPostProcessing {
    /// The provided post-process stages
    provided_post_processing: Arena<PostProcessingShader>,

    /// The objects tied to the render scale and swapchain size
    render_scale_objects: RenderScaleObjects,
}
