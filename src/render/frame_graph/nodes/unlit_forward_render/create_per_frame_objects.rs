use crate::{
    Result,
    render::{FixedRenderObjects, PerFrameObjectBuilder, frame_graph::UnlitForwardRenderNode},
};

impl UnlitForwardRenderNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        // Create the camera descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET,
        )?;

        // create the renderables descriptor set
        per_frame_objects.add_descriptor_set(
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET_LAYOUT,
            FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET,
        )
    }
}
