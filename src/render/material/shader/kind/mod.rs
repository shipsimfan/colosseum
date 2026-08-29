// rustdoc imports
#[allow(unused_imports)]
use crate::render::MaterialKind;

mod display;
mod is_compatible_with;

/// The types of materials that a shader can be used with
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    /// The shader is for unlit materials
    ///
    /// This can be used with [`MaterialKind::UnlitOpaque`] materials
    Unlit,

    /// The shader is for lit materials
    ///
    /// This can be used with [`MaterialKind::LitOpaque`] materials
    Lit,
}
