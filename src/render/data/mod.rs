mod skybox;

mod get;
mod new;
mod reset;
mod scene_reset;
mod set;

pub use skybox::*;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /// The skybox to render
    skybox: Skybox,
}
