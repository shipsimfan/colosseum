use crate::{Texture, Transform, Vertex, Window};
use alexandria::{Input, Vector4};

pub enum Mesh {
    Triangle(alexandria::Mesh<Vertex>),
    Line(alexandria::LineMesh<Vertex>),
}

pub struct MeshRenderer {
    mesh: Mesh,
    transform: Transform,
    tint: Vector4,
    texture: Option<Texture>,
}

impl Mesh {
    pub fn new<I: Input>(vertices: &[Vertex], indices: &[u32], window: &mut Window<I>) -> Self {
        Mesh::Triangle(alexandria::Mesh::new(vertices, indices, window.inner()).unwrap())
    }

    pub fn new_line<I: Input>(vertices: &[Vertex], strip: bool, window: &mut Window<I>) -> Self {
        Mesh::Line(alexandria::LineMesh::new(vertices, strip, window.inner()).unwrap())
    }

    pub fn render(&mut self) {
        match self {
            Mesh::Triangle(mesh) => mesh.render(),
            Mesh::Line(mesh) => mesh.render(),
        }
    }
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, texture: Option<Texture>) -> Self {
        MeshRenderer {
            mesh,
            transform: Transform::new(),
            tint: Vector4::ONE,
            texture,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn tint(&self) -> Vector4 {
        self.tint
    }

    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_tint(&mut self, new_tint: Vector4) {
        self.tint = new_tint;
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        window.set_object_buffer(*self.transform.transform(), self.tint);

        let texture = self.texture.as_ref().unwrap_or(window.default_texture());
        texture.set_active();
        self.mesh.render();
        texture.clear_active();
    }
}
