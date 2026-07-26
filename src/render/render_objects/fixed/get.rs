use crate::render::{FixedRenderObjects, Pipeline};

impl FixedRenderObjects {
    /// Get the list of pipelines created for frame graph nodes that don't use materials
    pub fn pipelines(&self) -> &[Pipeline] {
        &self.pipelines
    }
}
