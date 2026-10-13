use crate::render::ShaderKind;

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Unlit => "Unlit",
            ShaderKind::Lit => "Lit",
        }
        .fmt(f)
    }
}
