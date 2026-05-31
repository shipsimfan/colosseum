use alexandria::math::{Color3f, Linear};

mod get;
mod new;
mod reset;
mod scene_reset;
mod set;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /// The color to clear the screen to before rendering
    clear_color: Color3f<Linear>,
}
