use data_format::{Deserialize, Serialize};

/// The quality of shadows used in rendering
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowQuality {
    /// Low quality shadows
    Low = 0,

    /// Medium quality shadows
    #[default]
    Medium = 1,

    /// High quality shadows
    High = 2,
}
