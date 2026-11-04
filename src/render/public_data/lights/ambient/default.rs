use crate::render::AmbientLight;

impl Default for AmbientLight {
    fn default() -> Self {
        AmbientLight::new((0.2, 0.2, 0.24))
    }
}
