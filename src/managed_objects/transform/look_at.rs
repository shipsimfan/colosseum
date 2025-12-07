use crate::{
    Transform,
    math::{Quaternion, Vector3f},
};

impl Transform {
    /// Rotate this transform so it is looking at `target`
    pub fn look_at(&mut self, target: &Transform) {
        self.look_at_pos(target.position);
    }

    /// Rotate this transform so it is looking at `target` using `up`
    pub fn look_at_up(&mut self, target: &Transform, up: Vector3f) {
        self.look_at_pos_up(target.position, up);
    }

    /// Rotate this transform so it is looking at `target`
    pub fn look_at_pos(&mut self, target: Vector3f) {
        self.look_at_pos_up(target, Vector3f::UNIT_Y);
    }

    /// Rotate this transform so it is looking at `target` using `up`
    pub fn look_at_pos_up(&mut self, target: Vector3f, up: Vector3f) {
        self.set_rotation(Quaternion::look_at(target - self.position, up));
    }
}
