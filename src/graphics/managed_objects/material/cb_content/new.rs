use crate::{graphics::managed_objects::material::MaterialCbContent, math::Color3f};

impl MaterialCbContent {
    /// Create a new [`MaterialCbContent`]
    pub fn new(color: Color3f, specular_strength: f32) -> MaterialCbContent {
        MaterialCbContent {
            color,
            specular_strength,
        }
    }
}
