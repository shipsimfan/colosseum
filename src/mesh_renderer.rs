use crate::{Transform, Vertex, Window};
use alexandria::{Input, Topology, Vector4};

pub struct Mesh(alexandria::Mesh<Vertex>);

pub struct MeshRenderer {
    mesh: Mesh,
    transform: Transform,
    tint: Vector4,
}

impl Mesh {
    pub fn new<I: Input>(vertices: &[Vertex], indices: &[u32], window: &mut Window<I>) -> Self {
        Mesh(alexandria::Mesh::new(vertices, indices, window.inner()).unwrap())
    }

    pub fn set_topology(&mut self, topology: Topology) {
        self.0.set_topology(topology);
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.0.render(window.inner())
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
