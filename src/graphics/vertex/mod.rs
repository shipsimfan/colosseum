use crate::math::{Color3f, Vector3f};

mod lit_input_layout;
mod unlit_input_layout;

/// A vertex in a mesh
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// The position of the vertex
    pub position: Vector3f,

    /// The color of the vertex
    pub color: Color3f,
}
