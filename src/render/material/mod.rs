mod render;
mod shader;

mod new;

pub use shader::*;

pub(crate) use render::*;

/// A material being used in rendering
///
/// This is the material as it exists in the update job
pub struct Material {}
