use crate::util::{Arena, Handle};
use render_scale_objects::RenderScaleObjects;

mod anti_aliasing;
mod render_scale_objects;
mod shader;

mod bind;
mod clear;
mod new;
mod resize;
mod run;

pub use anti_aliasing::AntiAliasing;
pub use shader::PostProcessingShader;

/// A handle to a provided post-process stage
pub type PostProcessHandle = Handle<PostProcessingShader>;

/// The objects used during post processing
pub struct PostProcessing {
    /// The provided post-process stages
    provided_post_processing: Arena<PostProcessingShader>,

    /// The objects tied to the render scale and swapchain size
    render_scale_objects: RenderScaleObjects,

    /// The current render scale
    render_scale: f32,

    /// The type of anti-aliasing being used
    anti_aliasing: Option<AntiAliasing>,
}
