use crate::render::MaterialKind;

impl std::fmt::Display for MaterialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaterialKind::UnlitOpaque => "UnlitOpaque",
            MaterialKind::LitOpaque => "LitOpaque",
        }
        .fmt(f)
    }
}
