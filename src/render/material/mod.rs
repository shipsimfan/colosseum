mod id;
mod kind;
mod render;
mod shader;

mod new;

pub use id::*;
pub use kind::*;
pub use shader::*;

pub(crate) use render::*;

/// A material being used in rendering
///
/// This is the material as it exists in the update job
pub(crate) struct Material {}
