use crate::render::GraphicsDevice;
use alexandria::{Id, SlotMap, gpu::VulkanPipelineLayout};
use std::sync::Arc;

mod id;
mod unlit;

mod create;
mod get;
mod new;

pub use id::*;
pub use unlit::*;

/// The collection of materials available for rendering
pub struct Materials {
    /// The set of unlit materials that have been loaded
    unlit_materials: SlotMap<UnlitMaterial>,

    /// The ID of the default unlit material
    default_unlit_material: Id<UnlitMaterial>,

    /// The default unlit shader for unlit materials
    default_unlit_shader: Arc<UnlitShader>,

    /// The pipeline layout for unlit materials
    unlit_pipeline_layout: VulkanPipelineLayout,

    /// The device for creating pipelines
    device: Arc<GraphicsDevice>,
}
