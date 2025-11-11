use crate::graphics::ShaderInner;
use std::num::NonZeroU32;

impl ShaderInner {
    /// Gets the ID assigned to this shader
    pub(in crate::graphics) fn id(&self) -> NonZeroU32 {
        self.id
    }
}
