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
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, -0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, 0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(0.5, 0.5, -0.5),
                color: Color3::WHITE,
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
        ];
        const INDICES: &[u32] = &[
            0, 1, 2, 2, 3, 0, 6, 5, 4, 4, 7, 6, 8, 9, 10, 10, 11, 8, 14, 13, 12, 12, 15, 14, 16,
            17, 18, 18, 19, 16, 22, 21, 20, 20, 23, 22,
        ];

        unsafe { Mesh::new_unchecked(VERTICES, INDICES) }
    }
}
