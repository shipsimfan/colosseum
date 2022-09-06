use crate::{Mesh, Texture, Transform2D, Vertex, Window};
use alexandria::{Input, Vector4};

pub struct Sprite {
    mesh: Option<Mesh>,
    transform: Transform2D,
    tint: Vector4,
    texture: Option<Texture>,
}

impl Sprite {
    const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

    pub(crate) fn create_default_sprite_mesh<I: Input>(window: &mut Window<I>) -> Mesh {
        let vertices: [Vertex; 4] = [
            Vertex::new(-0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
            Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            Vertex::new(0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0),
        ];

        Mesh::new(&vertices, &Self::INDICES, window)
    }

    pub fn new(texture: Option<Texture>) -> Self {
        Sprite {
            mesh: None,
            transform: Transform2D::new(),
            tint: Vector4::ONE,
            texture,
        }
    }

    pub fn with_uv<I: Input>(
        window: &mut Window<I>,
        texture: Option<Texture>,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
    ) -> Self {
        let vertices: [Vertex; 4] = [
            Vertex::new(-0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, left, bottom),
            Vertex::new(-0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, left, top),
            Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, right, top),
            Vertex::new(0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0, right, bottom),
        ];

        Sprite {
            mesh: Some(Mesh::new(&vertices, &Self::INDICES, window)),
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
        let texture = self
            .texture
            .as_ref()
            .unwrap_or(window.default_texture())
            .clone();
        texture.set_active();
        match &mut self.mesh {
            Some(mesh) => mesh.render(),
            None => window.render_default_sprite_mesh(),
        }
        texture.clear_active();
    }
}
