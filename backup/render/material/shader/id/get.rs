use crate::render::{Shader, ShaderId, ShaderKind};
use alexandria::Id;
use std::sync::Arc;

impl ShaderId {
    /// Get the kind of shader this is
    pub(crate) fn kind(&self) -> ShaderKind {
        self.kind
    }

    /// Get the id of the shader in the list
    pub(crate) fn id(&self) -> Id<Arc<Shader>> {
        self.id
    }
}
