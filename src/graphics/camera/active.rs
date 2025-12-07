use crate::graphics::Camera;

impl Camera {
    /// Is this camera active?
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Enable this camera
    pub fn enable(&mut self) {
        self.active = true;
    }

    /// Disable this camera
    pub fn disable(&mut self) {
        self.active = false;
    }
}
