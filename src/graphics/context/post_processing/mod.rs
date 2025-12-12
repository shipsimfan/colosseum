use crate::graphics::util::{IndexBuffer, TextureSampler, VertexBuffer, VertexShader};
use quad::{POST_PROCESS_INPUT_LAYOUT, PostProcessingVertex, QUAD_INDICES, QUAD_VERTICES};
use win32::{ComPtr, d3d11::ID3D11DepthStencilState};

mod anti_aliasing;
mod quad;
mod shader;

mod new;
mod run;

pub use anti_aliasing::AntiAliasing;
pub use shader::PostProcessingShader;

/// Common elements to post-processing
pub(in crate::graphics::context) struct PostProcessing {
    /// The state describing how the depth stecil view should work for post process passes
    depth_stencil_state: ComPtr<ID3D11DepthStencilState>,

    /// The sampler to use for most post-process passes
    sampler: TextureSampler,

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
