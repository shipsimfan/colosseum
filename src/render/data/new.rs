use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub fn new() -> RenderData {
        RenderData {
            skybox: Skybox::default(),
            render_object_changes: Vec::new(),
            unlit_opaque_renderables: Vec::new(),
        }
    }
}
