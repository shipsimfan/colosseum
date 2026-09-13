use crate::render::frame_graph::{ShadowMapLight, ShadowMapNode};

impl<L: ShadowMapLight> std::fmt::Debug for ShadowMapNode<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShadowMapNode")
            .field("_phantom", &self._phantom)
            .finish()
    }
}
