use data_format::{Deserialize, Serialize};

/// The type of anti-aliasing to use when rendering the scene
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiAliasing {
    /// No anti-aliasing is applied
    None,

    /// Fast Approximate Anti-Aliasing (FXAA) is applied
    #[default]
    FXAA,
}
