use crate::{Mesh, Texture, Transform2D, Vertex, Window};
use alexandria::{Input, Vector4};

pub struct Sprite {
    mesh: Mesh,
    transform: Transform2D,
    tint: Vector4,
    texture: Option<Texture>,
}

impl Sprite {
    pub fn new<I: Input>(window: &mut Window<I>, texture: Option<Texture>) -> Self {
        const VERTICES: [Vertex; 4] = [
            Vertex::new(-0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
            Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            Vertex::new(0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0),
        ];

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

        Sprite {
            mesh: Mesh::new(&VERTICES, &INDICES, window),
            transform: Transform2D::new(),
            tint: Vector4::ONE,
            texture,
        }
    }

    pub fn transform(&self) -> &Transform2D {
        &self.transform
    }

    pub fn tint(&self) -> Vector4 {
        self.tint
    }

    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }

    pub fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }

    pub fn set_tint(&mut self, new_tint: Vector4) {
        self.tint = new_tint;
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        window.set_object_buffer(*self.transform.transform(), self.tint);
        self.texture.as_mut().map(|texture| texture.set_active());
        self.mesh.render();
        self.texture.as_mut().map(|texture| texture.clear_active());
    }
}
