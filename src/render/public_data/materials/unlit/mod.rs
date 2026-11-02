use alexandria::math::{Color4f, Linear};
use std::sync::Arc;

mod shader;

mod new;
mod set;

pub use shader::*;

/// A material that is not affected by lighting
pub struct UnlitMaterial {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// The shader used by the material
    pub shader: Arc<UnlitShader>,
}
