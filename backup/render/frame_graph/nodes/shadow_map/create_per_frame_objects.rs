use crate::{
    Result,
    render::{
        PerFrameObjectBuilder,
        frame_graph::{ShadowMapLight, ShadowMapNode},
    },
};

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Create needed per-frame resources for this node
    pub(in crate::render::frame_graph::nodes) fn create_per_frame_objects(
        per_frame_objects: &mut PerFrameObjectBuilder,
        index: usize,
    ) -> Result<()> {
        per_frame_objects.add_descriptor_set(
            format!("{} Light Descriptor Set {}", L::NAME, index),
            L::DESCRIPTOR_SET_LAYOUT,
            L::DESCRIPTOR_SET,
        )
    }
}
