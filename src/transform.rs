use graphics::{Matrix, Vector3};
use std::ops::Add;

pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,

    position_matrix: Matrix,
    rotation_matrix: Matrix,
    scale_matrix: Matrix,

    transform_matrix: Matrix,

    updated: bool,
}

impl Transform {
    pub fn new() -> Self {
        let position = Vector3::ZERO;
        let rotation = Vector3::ZERO;
        let scale = Vector3::ONE;

        let position_matrix = Matrix::translation(position.x(), position.y(), position.z());
        let rotation_matrix = Matrix::translation(rotation.x(), rotation.y(), rotation.z());
        let scale_matrix = Matrix::scale(scale.x(), scale.y(), scale.z());

        let transform_matrix = scale_matrix * rotation_matrix * position_matrix;

        Transform {
            position,
            rotation,
            scale,
            position_matrix,
            rotation_matrix,
            scale_matrix,
            transform_matrix,
            updated: true,
        }
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn rotation(&self) -> &Vector3 {
        &self.rotation
    }

    pub fn scale(&self) -> &Vector3 {
        &self.scale
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform_matrix
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn set_position(&mut self, new_position: Vector3) {
        self.position = new_position;
        self.position_matrix =
            Matrix::translation(self.position.x(), self.position.y(), self.position.z());
        self.update_transform_matrix();
    }

    pub fn set_rotation(&mut self, new_rotation: Vector3) {
        self.rotation = new_rotation;
        self.rotation_matrix =
            Matrix::rotation(self.rotation.x(), self.rotation.y(), self.rotation.z());
        self.update_transform_matrix();
    }

    pub fn set_scale(&mut self, new_scale: Vector3) {
        self.scale = new_scale;
        self.scale_matrix = Matrix::scale(self.scale.x(), self.scale.y(), self.scale.z());
        self.update_transform_matrix();
    }

    fn update_transform_matrix(&mut self) {
        self.transform_matrix = self.position_matrix * self.rotation_matrix * self.scale_matrix;
        self.updated = true;
    }
}

impl Add for &Transform {
    type Output = Transform;
    fn add(self, rhs: Self) -> Self::Output {
        let position = self.position + rhs.position;
        let rotation = self.rotation + rhs.rotation;
        let scale = self.scale * rhs.scale;

        let position_matrix = Matrix::translation(position.x(), position.y(), position.z());
        let rotation_matrix = Matrix::translation(rotation.x(), rotation.y(), rotation.z());
        let scale_matrix = Matrix::scale(scale.x(), scale.y(), scale.z());

        let transform_matrix = scale_matrix * rotation_matrix * position_matrix;

        Transform {
            position,
            rotation,
            scale,
            position_matrix,
            rotation_matrix,
            scale_matrix,
            transform_matrix,
            updated: true,
        }
    }
}
