use data_format::{Deserialize, Serialize};

mod size;

/// The quality of shadows used in rendering
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowQuality {
    /// Low quality shadows
    Low,

    /// Medium quality shadows
    #[default]
    Medium,

    /// High quality shadows
    High,
}
