use crate::render::{MaterialKind, ShaderKind};

impl ShaderKind {
    /// Is this [`ShaderKind`] compatible with the given [`MaterialKind`]?
    pub(crate) fn is_compatible_with(&self, material_kind: MaterialKind) -> bool {
        match (self, material_kind) {
            (ShaderKind::Unlit, MaterialKind::UnlitOpaque) => true,
        }
    }
}
