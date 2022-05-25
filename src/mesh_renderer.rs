use crate::{Transform, Vertex, Window};
use alexandria::{Input, Vector4};

pub enum Mesh {
    Triangle(alexandria::Mesh<Vertex>),
    Line(alexandria::LineMesh<Vertex>),
}

pub struct MeshRenderer {
    mesh: Mesh,
    transform: Transform,
    tint: Vector4,
}

impl Mesh {
    pub fn new<I: Input>(vertices: &[Vertex], indices: &[u32], window: &mut Window<I>) -> Self {
        Mesh::Triangle(alexandria::Mesh::new(vertices, indices, window.inner()).unwrap())
    }

    pub fn new_line<I: Input>(vertices: &[Vertex], strip: bool, window: &mut Window<I>) -> Self {
        Mesh::Line(alexandria::LineMesh::new(vertices, strip, window.inner()).unwrap())
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        let window = window.inner();
        match self {
            Mesh::Triangle(mesh) => mesh.render(window),
            Mesh::Line(mesh) => mesh.render(window),
        }
    }
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        MeshRenderer {
            mesh,
            transform: Transform::new(),
            tint: Vector4::ONE,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn tint(&self) -> Vector4 {
        self.tint
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_tint(&mut self, new_tint: Vector4) {
        self.tint = new_tint;
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        window.set_object_buffer(*self.transform.transform(), self.tint);
        self.mesh.render(window);
    }
}
