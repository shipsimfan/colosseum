use crate::update::components::Transform;
use alexandria::math::{Matrix4x4f, Quaternionf, Vector3f};

impl Transform {
    /// Get the position of the transform
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get a mutable reference to the position of the transform
    pub fn position_mut(&mut self) -> &mut Vector3f {
        self.dirty = true;
        &mut self.position
    }

    /// Get the x position of the transform
    pub fn x(&self) -> f32 {
        self.position.x
    }

    /// Get the y position of the transform
    pub fn y(&self) -> f32 {
        self.position.y
    }

    /// Get the z position of the transform
    pub fn z(&self) -> f32 {
        self.position.z
    }

    /// Get the rotation of the transform
    pub fn rotation(&self) -> Quaternionf {
        self.rotation
    }

    /// Get a mutable reference to the rotation of the transform
    pub fn rotation_mut(&mut self) -> &mut Quaternionf {
        self.dirty = true;
        &mut self.rotation
    }

    /// Get the x rotation of the transform
    pub fn rotation_x(&self) -> f32 {
        self.rotation.x
    }

    /// Get the y rotation of the transform
    pub fn rotation_y(&self) -> f32 {
        self.rotation.y
    }

    /// Get the z rotation of the transform
    pub fn rotation_z(&self) -> f32 {
        self.rotation.z
    }

    /// Get the w rotation of the transform
    pub fn rotation_w(&self) -> f32 {
        self.rotation.w
    }

    /// Get the scale of the transform
    pub fn scale(&self) -> Vector3f {
        self.scale
    }

    /// Get a mutable reference to the scale of the transform
    pub fn scale_mut(&mut self) -> &mut Vector3f {
        self.dirty = true;
        &mut self.scale
    }

    /// Get the x scale of the transform
    pub fn scale_x(&self) -> f32 {
        self.scale.x
    }

    /// Get the y scale of the transform
    pub fn scale_y(&self) -> f32 {
        self.scale.y
    }

    /// Get the z scale of the transform
    pub fn scale_z(&self) -> f32 {
        self.scale.z
    }

    /// Get the matrix of the transform
    pub(in crate::update) fn matrix(&mut self) -> Matrix4x4f {
        if self.dirty || self.camera {
            self.matrix = Matrix4x4f::from_translation(self.position)
                * Matrix4x4f::from_rotation(self.rotation)
                * Matrix4x4f::from_scale(self.scale);
            self.dirty = false;
            self.camera = false;
        }

        self.matrix
    }

    /// Get the camera matrix of the transform
    pub(in crate::update) fn camera_matrix(&mut self) -> Matrix4x4f {
        if self.dirty || !self.camera {
            self.matrix = Matrix4x4f::from_scale(self.scale)
                * Matrix4x4f::from_rotation(self.rotation.conjugate())
                * Matrix4x4f::from_translation(-self.position);
            self.dirty = false;
            self.camera = true;
        }

        self.matrix
    }
}
