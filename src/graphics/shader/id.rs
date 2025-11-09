use crate::graphics::Shader;
use std::num::NonZeroU32;

impl Shader {
    /// Gets the ID assigned to this shader
    pub(in crate::graphics) fn id(&self) -> NonZeroU32 {
        self.id
    }
}
