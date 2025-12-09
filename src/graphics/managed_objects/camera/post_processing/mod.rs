use crate::{
    graphics::util::{IndexBuffer, VertexBuffer, VertexShader},
    util::{Arena, Handle},
};
use quad::{POST_PROCESS_INPUT_LAYOUT, PostProcessingVertex, QUAD_INDICES, QUAD_VERTICES};
use render_scale_objects::RenderScaleObjects;

mod anti_aliasing;
mod quad;
mod render_scale_objects;
mod shader;

mod bind;
mod clear;
mod new;
mod resize;
mod run;

pub use anti_aliasing::AntiAliasing;
pub use shader::PostProcessingShader;
use win32::{
    ComPtr,
    d3d11::{ID3D11DepthStencilState, ID3D11SamplerState},
};

/// A handle to a provided post-process stage
pub type PostProcessHandle = Handle<PostProcessingShader>;

/// The objects used during post processing
pub struct PostProcessing {
    /// The provided post-process stages
    provided_post_processing: Arena<PostProcessingShader>,

    /// The objects tied to the render scale and swapchain size
    render_scale_objects: RenderScaleObjects,

    /// The state describing how the depth stecil view should work for post process passes
    depth_stencil_state: ComPtr<ID3D11DepthStencilState>,

    /// The sampler state to use for all post-process passes
    sampler_state: ComPtr<ID3D11SamplerState>,

    /// The vertex buffer for the quad
    vertex_buffer: VertexBuffer<PostProcessingVertex>,

    /// The index buffer for the quad
    index_buffer: IndexBuffer,

    /// The vertex shader used in all post-process passes
    vertex_shader: VertexShader,

    /// The shader to use for color correction
    color_correction_shader: PostProcessingShader,

    /// The shader to use for render scaling
    render_scale_shader: PostProcessingShader,
}
