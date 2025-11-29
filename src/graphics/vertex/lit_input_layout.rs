use crate::graphics::Vertex;
use win32::{
    d3d11::{D3D11_APPEND_ALIGNED_ELEMENT, D3D11_INPUT_CLASSIFICATION, D3D11_INPUT_ELEMENT_DESC},
    dxgi::DXGI_FORMAT,
};

impl Vertex {
    /// The input layout to use for all lit vertex shaders
    pub(in crate::graphics) const LIT_INPUT_LAYOUT: &[D3D11_INPUT_ELEMENT_DESC] = &[
        // Vertex elements
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"POSITION".as_ptr(),
            semantic_index: 0,
            format: DXGI_FORMAT::R32G32B32Float,
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
            instance_data_step_rate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"COLOR".as_ptr(),
            semantic_index: 0,
            format: DXGI_FORMAT::R32G32B32Float,
            input_slot: 0,
            aligned_byte_offset: D3D11_APPEND_ALIGNED_ELEMENT,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
            instance_data_step_rate: 0,
        },
        // Instance elements
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"INST_OBJECT".as_ptr(),
            semantic_index: 0,
            format: DXGI_FORMAT::R32G32B32A32Float,
            input_slot: 1,
            aligned_byte_offset: 0,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerInstanceData,
            instance_data_step_rate: 1,
        },
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"INST_OBJECT".as_ptr(),
            semantic_index: 1,
            format: DXGI_FORMAT::R32G32B32A32Float,
            input_slot: 1,
            aligned_byte_offset: D3D11_APPEND_ALIGNED_ELEMENT,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerInstanceData,
            instance_data_step_rate: 1,
        },
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"INST_OBJECT".as_ptr(),
            semantic_index: 2,
            format: DXGI_FORMAT::R32G32B32A32Float,
            input_slot: 1,
            aligned_byte_offset: D3D11_APPEND_ALIGNED_ELEMENT,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerInstanceData,
            instance_data_step_rate: 1,
        },
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: c"INST_OBJECT".as_ptr(),
            semantic_index: 3,
            format: DXGI_FORMAT::R32G32B32A32Float,
            input_slot: 1,
            aligned_byte_offset: D3D11_APPEND_ALIGNED_ELEMENT,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerInstanceData,
            instance_data_step_rate: 1,
        },
    ];
}
