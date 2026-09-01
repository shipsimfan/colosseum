use crate::{
    Result,
    render::{FixedRenderObjects, PerFrameObjectBuilder, frame_graph::QuantizationNode},
};

impl QuantizationNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the quantization descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::QUANTIZATION_DESCRIPTOR_SET,
        )
    }
}
