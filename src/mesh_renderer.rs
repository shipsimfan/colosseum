use crate::{Transform, Vertex, Window};
use graphics::Input;

pub struct Mesh(graphics::Mesh<Vertex>);

pub struct MeshRenderer {
    mesh: Mesh,
    transform: Transform,
}

impl Mesh {
    pub fn new<I: Input>(vertices: &[Vertex], indices: &[u32], window: &mut Window<I>) -> Self {
        Mesh(graphics::Mesh::new(vertices, indices, window.inner()).unwrap())
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
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        window.set_object_matrix(*self.transform.transform());
        self.mesh.render(window);
    }
}
