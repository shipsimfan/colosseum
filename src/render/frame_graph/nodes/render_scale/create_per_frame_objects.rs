use crate::{
    Result,
    render::{PerFrameObjectBuilder, frame_graph::RenderScaleNode},
};

impl RenderScaleNode {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        _: &mut PerFrameObjectBuilder,
    ) -> Result<()> {
        Ok(())
    }
}
