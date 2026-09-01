use crate::{
    Result,
    render::{FixedRenderObjects, PerFrameObjectBuilder, frame_graph::FxaaNode},
};

impl FxaaNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::FXAA_DESCRIPTOR_SET,
        )
    }
}
