use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};

impl FrameGraphResources {
    /// Get a resource based on its id
    pub fn get<'a>(&'a self, id: FrameGraphResourceId) -> FrameGraphResource<'a> {
        if id.is_external() {
            return (&self.external_resources[id.index()]).into();
        }

        panic!("only external resources are currently supported")
    }
}
