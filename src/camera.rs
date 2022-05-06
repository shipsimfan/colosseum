use crate::{Transform, Window};
use alexandria::{Input, Matrix};
use std::f32::consts::PI;

pub enum Projection {
    Perspective(f32, f32, f32),
    Orthographic(f32, f32, f32),
}

pub struct Camera {
    transform: Transform,
    projection_matrix: Matrix,
    total_matrix: Matrix,
}

impl Camera {
    pub fn new<I: Input>(window: &mut Window<I>) -> Self {
        let projection = Projection::Perspective(PI / 4.0, 0.01, 1000.0);
        let transform = Transform::new();
        let projection_matrix = projection.create_matrix(window);
        let total_matrix = *transform.transform() * projection_matrix;

        Camera {
            transform,
            projection_matrix,
            total_matrix,
        }
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        if self.transform.updated() {
            self.total_matrix = *self.transform.transform() * self.projection_matrix;
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
