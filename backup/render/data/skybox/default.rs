use crate::render::RenderSkybox;
use alexandria::math::Color3f;

impl Default for RenderSkybox {
    fn default() -> Self {
        RenderSkybox::SolidColor(Color3f::BLACK)
    }
}
