use win32::{
    d3d11::{D3D11_APPEND_ALIGNED_ELEMENT, D3D11_INPUT_CLASSIFICATION, D3D11_INPUT_ELEMENT_DESC},
    dxgi::DXGI_FORMAT,
};

use crate::math::Vector2f;

/// A vertex of the post processing quad
#[repr(C)]
pub(in crate::graphics::managed_objects::camera::post_processing) struct PostProcessingVertex {
    /// The position of the vertex
    position: Vector2f,

    /// The texture coordinate of the vertex
    uv: Vector2f,
}

pub(in crate::graphics::managed_objects::camera::post_processing) const POST_PROCESS_INPUT_LAYOUT:
    &[D3D11_INPUT_ELEMENT_DESC] = &[
    D3D11_INPUT_ELEMENT_DESC {
        semantic_name: c"POSITION".as_ptr(),
        semantic_index: 0,
        format: DXGI_FORMAT::R32G32Float,
        input_slot: 0,
        aligned_byte_offset: 0,
        input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
        instance_data_step_rate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        semantic_name: c"TEXCOORD".as_ptr(),
        semantic_index: 0,
        format: DXGI_FORMAT::R32G32Float,
        input_slot: 0,
        aligned_byte_offset: D3D11_APPEND_ALIGNED_ELEMENT,
        input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
        instance_data_step_rate: 0,
    },
];

/// The vertices making up the fullscreen post process quad
pub(in crate::graphics::managed_objects::camera::post_processing) const QUAD_VERTICES:
    &[PostProcessingVertex] = &[
    PostProcessingVertex {
        position: Vector2f::new(-1.0, -1.0),
        uv: Vector2f::new(0.0, 1.0),
    },
    PostProcessingVertex {
        position: Vector2f::new(1.0, -1.0),
        uv: Vector2f::new(1.0, 1.0),
    },
    PostProcessingVertex {
        position: Vector2f::new(1.0, 1.0),
        uv: Vector2f::new(1.0, 0.0),
    },
    PostProcessingVertex {
        position: Vector2f::new(-1.0, 1.0),
        uv: Vector2f::new(0.0, 0.0),
    },
];

/// The indices making up the fullscreen post process quad
pub(in crate::graphics::managed_objects::camera::post_processing) const QUAD_INDICES: &[u32] =
    &[0, 1, 2, 2, 3, 0];
