use crate::render::frame_graph::{ShadowMapLight, ShadowMapNode};
use std::marker::PhantomData;

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Create a new [`ShadowMapNode`]
    pub fn new() -> ShadowMapNode<L> {
        ShadowMapNode {
            _phantom: PhantomData,
        }
    }
}
