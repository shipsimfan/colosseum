use crate::{
    self as colosseum, Error, Result,
    graphics::{
        PostProcessingShader, ShaderSource,
        context::{
            PostProcessing,
            post_processing::{POST_PROCESS_INPUT_LAYOUT, QUAD_INDICES, QUAD_VERTICES},
        },
        util::{IndexBuffer, VertexBuffer, VertexShader},
    },
};
use colosseum_macros::compile_shader_file;
use win32::{
    ComPtr, FALSE,
    d3d11::{D3D11_DEPTH_STENCIL_DESC, D3D11_SAMPLER_DESC, ID3D11Device},
    try_hresult,
};

const VERTEX_SHADER: ShaderSource = compile_shader_file!("vertex_shader.hlsl", "vs_5_0", "main");
const COLOR_CORRECITON_SHADER: ShaderSource =
    compile_shader_file!("color_correction.hlsl", "ps_5_0", "main");
const RENDER_SCALE_SHADER: ShaderSource =
    compile_shader_file!("render_scale.hlsl", "ps_5_0", "main");

impl PostProcessing {
    /// Create a set of global [`PostProcessing`] objects
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        // Create depth stencil state
        let depth_stencil_desc = D3D11_DEPTH_STENCIL_DESC {
            depth_enable: FALSE,
            ..Default::default()
        };
        let depth_stencil_state = ComPtr::new_in(|depth_stencil_state| {
            try_hresult!(
                device.create_depth_stencil_state(&depth_stencil_desc, depth_stencil_state)
            )
        })
        .map_err(|error| {
            Error::new_inner("unable to create post process depth stencil state", error)
        })?;

        // Create sampler
        let sampler_desc = D3D11_SAMPLER_DESC::default();
        let sampler_state = ComPtr::new_in(|sampler_state| {
            try_hresult!(device.create_sampler_state(&sampler_desc, sampler_state))
        })
        .map_err(|error| Error::new_inner("unable to create post process sampler state", error))?;

        // Create quad mesh
        let vertex_buffer = VertexBuffer::new(QUAD_VERTICES, 0, device)?;
        let index_buffer = IndexBuffer::new(QUAD_INDICES, device)?;

        // Create shaders
        let vertex_shader =
            VertexShader::new(VERTEX_SHADER.content(), POST_PROCESS_INPUT_LAYOUT, device)?;

        let color_correction_shader = PostProcessingShader::new(&COLOR_CORRECITON_SHADER, device)?;
        let render_scale_shader = PostProcessingShader::new(&RENDER_SCALE_SHADER, device)?;

        Ok(PostProcessing {
            depth_stencil_state,
            sampler_state,
            vertex_buffer,
            index_buffer,
            vertex_shader,
            color_correction_shader,
            render_scale_shader,
        })
    }
}
