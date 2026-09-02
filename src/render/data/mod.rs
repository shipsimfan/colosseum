use crate::render::{Material, Mesh};
use alexandria::{
    Id,
    gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFence},
};
use std::sync::Arc;

mod anti_aliasing;
mod camera;
mod lighting;
mod local_buffer;
mod object;
mod remove_confirm;
mod render_object_change;
mod skybox;

mod add;
mod apply;
mod get;
mod new;
mod reserve;
mod reset;
mod set;
mod wait;

pub use anti_aliasing::*;
pub use skybox::*;

pub(crate) use lighting::*;
pub(crate) use object::*;
pub(crate) use remove_confirm::*;
pub(crate) use render_object_change::*;

pub(in crate::render) use camera::*;
pub(in crate::render) use local_buffer::*;

/// The information needed for a renderable
pub(crate) type Renderable = (Id<Material>, Id<Mesh>, usize);

/// The data required to execute a render job
pub(crate) struct RenderData {
    /** Render Objects **/

    /// The changes to the render objects in use
    render_object_changes: Vec<RenderObjectChange>,

    /// The objects whose removals have been confirmed, and the memory can be freed
    confirmed_removals: Vec<RenderObjectRemoveConfirm>,

    /// The fence to wait on for the completion of copy operations to the GPU
    copy_fence: VulkanFence,

    /// Were the commands for copying actually sent this frame?
    copy_commands_sent: bool,

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

    /// The camera data for the current frame
    camera: LocalDataBuffer<RenderCamera>,

    /// The data about lighting for the current frame
    lighting: LightingData,

    /// The set of unlit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not require any lighting
    /// calculations or transparency
    unlit_opaque_renderables: Vec<Renderable>,

    /// The set of lit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not transparency but do use
    /// lighting
    lit_opaque_renderables: Vec<Renderable>,

    /// The buffer for object data
    renderable_buffer: LocalDataBuffer<ObjectData>,

    /// The device to use when allocating memory
    device: VulkanDevice,

    /// The memory properties of the device
    memory_properties: Arc<VulkanAdapterMemoryProperties>,
}
