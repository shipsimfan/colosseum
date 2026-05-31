use crate::Window;
use alexandria::math::Vector2u;

impl Window {
    /// Gets the current size of the window
    pub fn size(&self) -> Vector2u {
        self.shared.size()
    }
}
