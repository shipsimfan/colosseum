use doubled::DoubledRenderData;
use renderable_list::RenderableList;

mod camera;
mod doubled;
mod remove_confirm;
mod render_object_change;
mod renderable_list;
mod skybox;

mod add;
mod apply;
mod get;
mod new;
mod reset;
mod set;

pub use skybox::*;

pub(crate) use camera::*;
pub(crate) use remove_confirm::*;
pub(crate) use render_object_change::*;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /** Render Objects **/

    /// The changes to the render objects in use
    render_object_changes: Vec<RenderObjectChange>,

    /// The objects whose removals have been confirmed, and the memory can be freed
    confirmed_removals: Vec<RenderObjectRemoveConfirm>,

    /** Scene Data **/

    /// The skybox to render
    skybox: Skybox,

    /// The render data that exists in two copies for each frame, so that one copy can be used for
    /// rendering while the other is being updated
    doubled: [DoubledRenderData; 2],

    /// The index of the doubled render data that is currently being used for rendering
    current_doubled_index: usize,
}
