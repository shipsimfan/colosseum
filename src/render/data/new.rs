use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub fn new() -> RenderData {
        RenderData {
            skybox: Skybox::default(),
            material_changes: Vec::new(),
            renderables: Vec::new(),
        }
    }
}
