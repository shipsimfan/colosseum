use crate::graphics::{MeshInner, Vertex};
use std::{borrow::Cow, cell::OnceCell};

impl MeshInner {
    /// Create a new [`MeshInner`]
    pub(in crate::graphics::mesh) fn new(
        vertices: Cow<'static, [Vertex]>,
        indices: Cow<'static, [u32]>,
    ) -> Self {
        MeshInner {
            vertices,
            indices,
            buffers: OnceCell::new(),
        }
    }
}
