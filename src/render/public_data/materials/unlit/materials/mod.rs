use crate::render::{UnlitMaterial, UnlitShader};
use alexandria::{Id, SlotMap, gpu::VulkanPipelineLayout};
use std::sync::Arc;

mod create;
mod get;
mod new;

/// The items related to unlit materials
pub(in crate::render::public_data::materials) struct UnlitMaterials {
    /// The set of unlit materials that have been loaded
    materials: SlotMap<UnlitMaterial>,

    /// The ID of the default unlit material
    default_material: Id<UnlitMaterial>,

    /// The default unlit shader for unlit materials
    default_shader: Arc<UnlitShader>,

    /// The pipeline layout for unlit materials
    pipeline_layout: VulkanPipelineLayout,
}
