use crate::{
    Result,
    render::{FixedRenderObjects, PerFrameObjectBuilder, frame_graph::ShadowMapNode},
};

impl ShadowMapNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
        index: usize,
    ) -> Result<()> {
        per_frame_objects.add_descriptor_set(
            format!("Directional Light Descriptor Set {}", index),
            FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::DIRECTIONAL_LIGHT_DESCRIPTOR_SET,
        )?;
        per_frame_objects.add_descriptor_set(
            format!("Spot Light Descriptor Set {}", index),
            FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::SPOT_LIGHT_DESCRIPTOR_SET,
        )
    }
}
