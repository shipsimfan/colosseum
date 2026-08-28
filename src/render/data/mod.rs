use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorSet};
use doubled::DoubledRenderData;
use renderable_list::RenderableList;

mod anti_aliasing;
mod camera;
mod doubled;
mod object;
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

pub use anti_aliasing::*;
pub use skybox::*;

pub(crate) use camera::*;
pub(crate) use object::*;
pub(crate) use remove_confirm::*;
pub(crate) use render_object_change::*;

/// The data required to execute a render job
pub(crate) struct RenderData {
    /** Render Objects **/

    /// The changes to the render objects in use
    render_object_changes: Vec<RenderObjectChange>,

    /// The objects whose removals have been confirmed, and the memory can be freed
    confirmed_removals: Vec<RenderObjectRemoveConfirm>,

    /// The descriptor pool containing the descriptors sets in this render data
    #[allow(unused)]
    descriptor_pool: VulkanDescriptorPool,

    /// The descriptor sets used by post-processing nodes
    post_process_descriptor_sets: Vec<VulkanDescriptorSet>,

    /** Render Settings **/

    /// The scale to render the scene at
    render_scale: f32,

    /// The gamma to render the scene with
    gamma: f32,

    /// The exposure to render the scene with
    exposure: f32,

    /// The contrast to render the scene with
    contrast: f32,

    /// The saturation to render the scene with
    saturation: f32,

    /// The type of anti-aliasing to use when rendering the scene
    anti_aliasing: AntiAliasingMode,

    /** Scene Data **/

    /// The skybox to render
    skybox: Skybox,

    /// The render data that exists in two copies for each frame, so that one copy can be used for
    /// rendering while the other is being updated
    doubled: [DoubledRenderData; 2],

    /// The index of the doubled render data that is currently being used for rendering
    current_doubled_index: usize,
}

impl RenderData {
    pub(in crate::render) const TONE_MAP_DESCRIPTOR_SET: usize = 0;
    pub(in crate::render) const QUANTIZATION_DESCRIPTOR_SET: usize = 1;
    pub(in crate::render) const FXAA_DESCRIPTOR_SET: usize = 2;
}
