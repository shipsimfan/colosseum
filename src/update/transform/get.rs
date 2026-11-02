use crate::update::Transform;
use alexandria::math::Matrix4x4f;

impl Transform {
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
    pub(crate) fn matrix(&self) -> Matrix4x4f {
        Matrix4x4f::from_translation(self.position)
            * Matrix4x4f::from_rotation(self.rotation)
            * Matrix4x4f::from_scale(self.scale)
    }
}
