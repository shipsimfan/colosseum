use alexandria::math::{Color3f, Linear, Vector3f};

mod get;
mod new;
mod set;

/// A vertex in a mesh
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// The position of the vertex
    position: Vector3f,

    /// The color of the vertex
    color: Color3f<Linear>,
}
