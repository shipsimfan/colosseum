use crate::Window;
use alexandria::{Input, Matrix, Vector3, Vector4};
use std::f32::consts::PI;

pub enum Projection {
    Perspective(f32, f32, f32),
    Orthographic(f32, f32, f32),
}

pub struct Camera {
    position: Vector3,
    rotation: Vector3,
    projection_matrix: Matrix,
    total_matrix: Matrix,
    updated: bool,
}

fn calculate_lookahead(position: Vector3, rotation: &Vector3) -> Matrix {
    let rotation_matrix = Matrix::rotation(rotation.x(), rotation.y(), rotation.z());
    let target = (rotation_matrix * Vector4::FORWARD).normal();

    let right = (rotation_matrix * Vector4::RIGHT).xyz();
    let forward = (rotation_matrix * Vector4::FORWARD).xyz();
    let up = forward.cross(right);

    let target = position + target.xyz();
    Matrix::look_at(position, target, up)
}

impl Camera {
    pub fn new<I: Input>(window: &mut Window<I>) -> Self {
        let projection = Projection::Perspective(PI / 4.0, 0.01, 1000.0);
        let projection_matrix = projection.create_matrix(window);

        let position = Vector3::ZERO;
        let rotation = Vector3::ZERO;

        let total_matrix = projection_matrix * calculate_lookahead(position, &rotation);

        Camera {
            position,
            rotation,
            projection_matrix,
            total_matrix,
            updated: false,
        }
    }

    pub fn position(&self) -> Vector3 {
        -self.position
    }

    pub fn rotation(&self) -> Vector3 {
        self.rotation
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.position = -position;
        self.updated = true;
    }

    pub fn set_rotation(&mut self, rotation: Vector3) {
        self.rotation = rotation;
        self.updated = true;
    }

    pub fn set_projection<I: Input>(&mut self, projection: Projection, window: &mut Window<I>) {
        self.projection_matrix = projection.create_matrix(window);
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        if self.updated {
            self.updated = false;
            self.total_matrix =
                self.projection_matrix * calculate_lookahead(self.position, &self.rotation);
        }

        window.set_camera_matrix(self.total_matrix);
    }
}

impl Projection {
    pub fn perspective(fovy: f32, near: f32, far: f32) -> Self {
        Projection::Perspective(fovy, near, far)
    }

    pub fn orthographic(width: f32, near: f32, far: f32) -> Self {
        Projection::Orthographic(width, near, far)
    }

    pub fn create_matrix<I: Input>(&self, window: &mut Window<I>) -> Matrix {
        let aspect = window.width() / window.height();

        match self {
            Projection::Perspective(fovy, near, far) => {
                Matrix::perspective(*fovy, aspect, *near, *far)
            }
            Projection::Orthographic(width, near, far) => {
                Matrix::orthographic(*width, *width / aspect, *near, *far)
            }
        }
    }
}
