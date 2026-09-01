use crate::{
    Result,
    render::{FixedRenderObjects, PerFrameObjectBuilder, frame_graph::LitForwardRenderNode},
};

impl LitForwardRenderNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the lighting descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::LIGHTING_DESCRIPTOR_SET,
        )
    }
}
