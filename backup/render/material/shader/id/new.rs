use crate::render::{Shader, ShaderId, ShaderKind};
use alexandria::Id;
use std::sync::Arc;

impl ShaderId {
    /// Create a new [`ShaderId`]
    pub(crate) fn new(kind: ShaderKind, id: Id<Arc<Shader>>) -> ShaderId {
        ShaderId { kind, id }
    }
}
