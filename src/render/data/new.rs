use crate::render::{RenderData, Skybox};

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub fn new() -> RenderData {
        RenderData {
            skybox: Skybox::default(),
        }
    }
}
