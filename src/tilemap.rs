use crate::{Texture, Transform2D, Vertex, Window};
use alexandria::{Input, Vector2, Vector4};

pub struct Tilemap {
    mesh: alexandria::Mesh<Vertex>,
    transform: Transform2D,
    tint: Vector4,
    texture: Option<Texture>,

    indices: Vec<u32>,
    vertices: Box<[Vertex]>,
    render_state: Box<[Option<usize>]>,

    vertices_need_update: bool,
    indices_need_update: bool,

    width: usize,
    height: usize,
}

impl Tilemap {
    pub fn new<I: Input>(
        width: usize,
        height: usize,
        texture: Option<Texture>,
        window: &mut Window<I>,
    ) -> Self {
        let mut indices = Vec::with_capacity(width * height * 6);
        let mut vertices = Vec::with_capacity(width * height * 4);
        let mut render_state = Vec::with_capacity(width * height);

        for y in 0..height {
            let y = y as f32;
            for x in 0..width {
                let x = x as f32;

                vertices.push(Vertex::new(x, y, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0));
                vertices.push(Vertex::new(x, y + 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0));
                vertices.push(Vertex::new(
                    x + 1.0,
                    y + 1.0,
                    0.0,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                ));
                vertices.push(Vertex::new(x + 1.0, y, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0));
                render_state.push(None);
            }
        }

        indices.push(0);

        Tilemap {
            mesh: alexandria::Mesh::new(&vertices, &indices, window.inner()).unwrap(),
            transform: Transform2D::new(),
            tint: Vector4::ONE,
            texture,
            indices,
            vertices: vertices.into_boxed_slice(),
            render_state: render_state.into_boxed_slice(),
            width,
            height,
            vertices_need_update: false,
            indices_need_update: false,
        }
    }

    pub fn transform(&self) -> &Transform2D {
        &self.transform
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_visible(&mut self, x: usize, y: usize, visible: bool) {
        assert!(x < self.width);
        assert!(y < self.height);

        let state_index = x + y * self.width;

        if self.render_state[state_index].is_some() == visible {
            return;
        }

        self.indices_need_update = true;

        let changed_index = match self.render_state[state_index] {
            Some(index) => {
                self.indices.drain(index..index + 6);
                self.render_state[state_index] = None;

                if self.indices.len() == 0 {
                    self.indices.push(0);
                    return;
                }

                index
            }
            None => {
                if self.indices.len() == 1 {
                    self.indices.pop();
                }

                self.render_state[state_index] = Some(self.indices.len());
                let index = (state_index * 4) as u32;
                self.indices
                    .extend(&[index, index + 1, index + 2, index + 2, index + 3, index]);
                return;
            }
        };

        for i in 0..self.render_state.len() {
            if i == state_index {
                continue;
            }

            match &mut self.render_state[i] {
                Some(index) => {
                    if *index > changed_index {
                        *index -= 6;
                    }
                }
                None => {}
            }
        }
    }

    pub fn set_tile_uv(&mut self, x: usize, y: usize, uv: [Vector2; 4]) {
        assert!(x < self.width);
        assert!(y < self.height);

        let index = (x + y * self.width) * 4;

        for i in 0..4 {
            *self.vertices[index + i].uv_mut() = uv[i];
        }

        self.vertices_need_update = true;
    }

    pub fn set_tile_color(&mut self, x: usize, y: usize, color: Vector4) {
        assert!(x < self.width);
        assert!(y < self.height);

        let index = (x + y * self.width) * 4;

        for i in 0..4 {
            *self.vertices[index + i].color_mut() = color;
        }

        self.vertices_need_update = true;
    }

    pub fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }

    pub fn set_tint(&mut self, tint: Vector4) {
        self.tint = tint;
    }

    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.indices.push(0);
        self.indices_need_update = true;

        for render_state in self.render_state.iter_mut() {
            *render_state = None;
        }
    }

    pub fn update_mesh<I: Input>(&mut self, window: &mut Window<I>) {
        if self.indices_need_update {
            self.mesh
                .update_indices(&self.indices, window.inner())
                .unwrap();
            self.indices_need_update = false;
        }

        if self.vertices_need_update {
            self.mesh
                .update_vertices(&self.vertices, window.inner())
                .unwrap();
            self.vertices_need_update = false;
        }
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        if self.indices.len() > 1 {
            window.set_object_buffer(*self.transform.transform(), self.tint);
            let texture = self.texture.as_ref().unwrap_or(window.default_texture());
            texture.set_active();
            self.mesh.render();
            texture.clear_active();
        }
    }
}
