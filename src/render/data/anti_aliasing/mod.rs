use data_format::{Deserialize, Serialize};

/// The type of anti-aliasing to use when rendering the scene
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiAliasingMode {
    /// No anti-aliasing is applied
    #[default]
    None,

    /// Fast Approximate Anti-Aliasing (FXAA) is applied
    FXAA,
}
