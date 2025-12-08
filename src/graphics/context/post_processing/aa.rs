use data_format::{Deserialize, Serialize};

/// The available types of anti-aliasing
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum AntiAliasing {
    /// Fast approximate anti-aliasing
    FXAA,
}
