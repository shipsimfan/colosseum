use crate::update::Skybox;
use alexandria::math::Color3f;

impl Default for Skybox {
    fn default() -> Self {
        Skybox::SolidColor(Color3f::BLACK)
    }
}
