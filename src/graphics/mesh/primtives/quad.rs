use crate::{
    graphics::{Mesh, MeshPrimitives, Vertex},
    math::{Color3, Vector3},
};

impl MeshPrimitives {
    /// Creates a 1x1 quad in the XY plane facing positive-z
    pub fn quad() -> Mesh {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.0),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.0),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.0),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.0),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
        ];
        const INDICES: &[u32] = &[2, 1, 0, 0, 3, 2];

        unsafe { Mesh::new_unchecked(VERTICES, INDICES) }
    }
}
