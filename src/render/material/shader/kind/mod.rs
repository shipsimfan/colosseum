// rustdoc imports
#[allow(unused_imports)]
use crate::render::MaterialKind;

mod display;
mod is_compatible_with;

/// The types of materials that a shader can be used with
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    /// The material is unlit
    ///
    /// This type of material is run in a forward pass without any lighting calculations
    ///
    /// This can be used with [`MaterialKind::UnlitOpaque`] materials
    Unlit,
}
