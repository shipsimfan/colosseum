use crate::render::RenderData;
use alexandria::math::Color3f;

impl RenderData {
    /// Reset the render data to its default state for a new frame
    pub fn scene_reset(&mut self) {
        self.clear_color = Color3f::BLACK;
    }
}
