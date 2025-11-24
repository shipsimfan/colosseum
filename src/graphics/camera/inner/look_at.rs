use crate::{
    graphics::CameraInner,
    math::{Transform, Vector3f},
};

impl CameraInner {
    /// Rotate this transform so it is looking at `target`
    pub fn look_at(&mut self, target: &Transform) {
        self.transform.look_at(target);
    }

    /// Rotate this transform so it is looking at `target` using `up`
    pub fn look_at_up(&mut self, target: &Transform, up: Vector3f) {
        self.transform.look_at_up(target, up);
    }

    /// Rotate this transform so it is looking at `target`
    pub fn look_at_pos(&mut self, target: Vector3f) {
        self.transform.look_at_pos(target);
    }

    /// Rotate this transform so it is looking at `target` using `up`
    pub fn look_at_pos_up(&mut self, target: Vector3f, up: Vector3f) {
        self.transform.look_at_pos_up(target, up);
    }
}
