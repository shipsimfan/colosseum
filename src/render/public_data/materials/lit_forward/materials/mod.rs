use crate::render::{LitForwardMaterial, LitForwardShader};
use alexandria::{Id, SlotMap, gpu::VulkanPipelineLayout};
use std::sync::Arc;

mod create;
mod get;
mod new;

/// The items related to lit materials using forward rendering
pub(in crate::render::public_data::materials) struct LitForwardMaterials {
    /// The set of lit forward materials that have been loaded
    materials: SlotMap<LitForwardMaterial>,

    /// The ID of the default lit forward material
    default_material: Id<LitForwardMaterial>,

    /// The default lit forward shader for lit forward materials
    default_shader: Arc<LitForwardShader>,

    /// The pipeline layout for lit materials in the forward render pass
    pipeline_layout: VulkanPipelineLayout,
}
