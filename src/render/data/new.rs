use crate::render::RenderData;
use alexandria::math::Color3f;

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub fn new() -> RenderData {
        RenderData {
            clear_color: Color3f::BLACK,
        }
    }
}
