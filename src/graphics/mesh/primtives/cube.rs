use crate::{
    graphics::{Mesh, MeshPrimitives, Vertex},
    math::{Color3, Vector3},
};

impl MeshPrimitives {
    /// Creates a 1x1x1 cube
    pub fn cube() -> Mesh {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: Vector3::new(-0.5, -0.5, -0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, -0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, -0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, -0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.5),
                color: Color3::WHITE,
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.5),
                color: Color3::WHITE,
            },
        ];
        const INDICES: &[u32] = &[
            0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2,
            6, 4, 5, 0, 0, 5, 1,
        ];

        unsafe { Mesh::new_unchecked(VERTICES, INDICES) }
    }
}
