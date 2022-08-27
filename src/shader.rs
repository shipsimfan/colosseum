use crate::Window;
use alexandria::{Input, Matrix, Vector2, Vector3, Vector4};

pub struct Shader {
    shader: alexandria::Shader,
}

#[repr(C)]
pub struct Vertex {
    position: Vector4,
    color: Vector4,
    uv: Vector2,
}

#[repr(C)]
pub struct ObjectBuffer {
    matrix: Matrix,
    tint: Vector4,
}

const VERTEX_LAYOUT: [(&str, alexandria::Format); 3] = [
    ("POSITION", alexandria::Format::R32G32B32A32Float),
    ("COLOR", alexandria::Format::R32G32B32A32Float),
    ("TEXCOORD", alexandria::Format::R32A32Float),
];

pub fn new_default<I: Input>(window: &mut Window<I>) -> Shader {
    Shader {
        shader: alexandria::Shader::new(
            include_str!("default_shader.acsl"),
            &VERTEX_LAYOUT,
            window.inner(),
        )
        .unwrap(),
    }
}

impl Shader {
    pub fn new<S: AsRef<str>, I: Input>(code: S, window: &mut Window<I>) -> Self {
        Shader {
            shader: alexandria::Shader::new(code, &VERTEX_LAYOUT, window.inner()).unwrap(),
        }
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        self.shader.set_active();
        window.update_camera_buffer();
    }
}

impl Vertex {
    pub fn new_vertex(position: Vector3, color: Vector4, uv: Vector2) -> Self {
        Vertex {
            position: Vector4::new(position.x(), position.y(), position.z(), 1.0),
            color,
            uv,
        }
    }

    pub const fn new(
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        uv0: f32,
        uv1: f32,
    ) -> Self {
        Vertex {
            position: Vector4::new(x, y, z, 1.0),
            color: Vector4::new(r, g, b, a),
            uv: Vector2::new(uv0, uv1),
        }
    }

    pub fn position(&self) -> &Vector4 {
        &self.position
    }

    pub fn color(&self) -> &Vector4 {
        &self.color
    }

    pub fn uv(&self) -> &Vector2 {
        &self.uv
    }

    pub fn position_mut(&mut self) -> &mut Vector4 {
        &mut self.position
    }

    pub fn color_mut(&mut self) -> &mut Vector4 {
        &mut self.color
    }

    pub fn uv_mut(&mut self) -> &mut Vector2 {
        &mut self.uv
    }
}

impl ObjectBuffer {
    pub fn new(matrix: Matrix, tint: Vector4) -> Self {
        ObjectBuffer { matrix, tint }
    }
}
