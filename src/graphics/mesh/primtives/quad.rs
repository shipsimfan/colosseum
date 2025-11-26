use crate::{
    graphics::{Mesh, MeshPrimitives, Vertex},
    math::{Color3, Vector3},
};

impl MeshPrimitives {
    /// Creates a 1x1 quad facing the positive x-axis
    pub fn quad() -> Mesh {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.0),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.0),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.0),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.0),
                color: Color3::WHITE,
            },
        ];
        const INDICES: &[u32] = &[0, 1, 2, 2, 3, 0];

        unsafe { Mesh::new_unchecked(VERTICES, INDICES) }
    }
}
