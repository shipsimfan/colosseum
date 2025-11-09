use crate::math::Vector3f;

mod input_layout;

/// A vertex in a mesh
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// The position of the vertex
    pub position: Vector3f,

    /// The color of the vertex
    pub color: Vector3f,
}
