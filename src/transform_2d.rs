use alexandria::{Matrix, Vector2};

pub struct Transform2D {
    position: Vector2,
    rotation: f32,
    scale: Vector2,
    z_order: f32,

    position_matrix: Matrix,
    rotation_matrix: Matrix,
    scale_matrix: Matrix,

    transform_matrix: Matrix,

    updated: bool,
}

impl Transform2D {
    pub fn new() -> Self {
        let position = Vector2::ZERO;
        let rotation = 0.0;
        let scale = Vector2::ONE;
        let z_order = 0.0;

        let position_matrix = Matrix::translation(position.x(), position.y(), z_order);
        let rotation_matrix = Matrix::rotation_z(rotation);
        let scale_matrix = Matrix::scale(scale.x(), scale.y(), 1.0);

        let transform_matrix = scale_matrix * rotation_matrix * position_matrix;

        Transform2D {
            position,
            rotation,
            scale,
            z_order,
            position_matrix,
            rotation_matrix,
            scale_matrix,
            transform_matrix,
            updated: true,
        }
    }

    pub fn position(&self) -> &Vector2 {
        &self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn scale(&self) -> &Vector2 {
        &self.scale
    }

    pub fn z_order(&self) -> f32 {
        self.z_order
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform_matrix
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn set_position(&mut self, new_position: Vector2) {
        self.position = new_position;
        self.position_matrix =
            Matrix::translation(self.position.x(), self.position.y(), self.z_order);
        self.update_transform_matrix();
    }

    pub fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
        self.rotation_matrix = Matrix::rotation_z(self.rotation);
        self.update_transform_matrix();
    }

    pub fn set_scale(&mut self, new_scale: Vector2) {
        self.scale = new_scale;
        self.scale_matrix = Matrix::scale(self.scale.x(), self.scale.y(), 1.0);
        self.update_transform_matrix();
    }

    pub fn set_z_order(&mut self, new_z_order: f32) {
        self.z_order = new_z_order;
        self.position_matrix =
            Matrix::translation(self.position.x(), self.position.y(), self.z_order);
        self.update_transform_matrix();
    }

    fn update_transform_matrix(&mut self) {
        self.transform_matrix = self.position_matrix * self.rotation_matrix * self.scale_matrix;
        self.updated = true;
    }
}
