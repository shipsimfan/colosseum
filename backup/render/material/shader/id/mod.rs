use crate::render::{Shader, ShaderKind};
use alexandria::Id;
use std::sync::Arc;

mod display;
mod get;
mod new;

/// The identifier for a shader
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId {
    /// The kind of shader this is
    kind: ShaderKind,

    /// The id of the shader in the list
    id: Id<Arc<Shader>>,
}
