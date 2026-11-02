use crate::update::Transform;
use alexandria::math::{Quaternionf, Vector3f};

impl Transform {
    /// Set the position of the [`Transform`]
    pub fn set_position<V: Into<Vector3f>>(&mut self, position: V) {
        self.position = position.into();
    }

    /// Set the x position of the [`Transform`]
    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    /// Set the y position of the [`Transform`]
    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    /// Set the z position of the [`Transform`]
    pub fn set_z(&mut self, z: f32) {
        self.position.z = z;
    }

    /// Set the rotation of the [`Transform`]
    pub fn set_rotation<Q: Into<Quaternionf>>(&mut self, rotation: Q) {
        self.rotation = rotation.into();
    }

    /// Set the x rotation of the [`Transform`]
    pub fn set_rotation_x(&mut self, x: f32) {
        self.rotation.x = x;
    }

    /// Set the y rotation of the [`Transform`]
    pub fn set_rotation_y(&mut self, y: f32) {
        self.rotation.y = y;
    }

    /// Set the z rotation of the [`Transform`]
    pub fn set_rotation_z(&mut self, z: f32) {
        self.rotation.z = z;
    }

    /// Set the scale of the [`Transform`]
    pub fn set_scale<S: Into<Vector3f>>(&mut self, scale: S) {
        self.scale = scale.into();
    }

    /// Set the x scale of the [`Transform`]
    pub fn set_scale_x(&mut self, x: f32) {
        self.scale.x = x;
    }

    /// Set the y scale of the [`Transform`]
    pub fn set_scale_y(&mut self, y: f32) {
        self.scale.y = y;
    }

    /// Set the z scale of the [`Transform`]
    pub fn set_scale_z(&mut self, z: f32) {
        self.scale.z = z;
    }
}
