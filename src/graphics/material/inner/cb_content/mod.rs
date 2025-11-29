use crate::math::Color3f;

mod new;

/// The content of the material constant buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::material::inner) struct MaterialCbContent {
    /// The color applied to all objects using this material
    pub color: Color3f,

    /// The strength of the bright spot appearing on objects
    pub specular_strength: f32,
}
