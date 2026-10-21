use crate::render::{
    FrameIndex,
    frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources},
};

impl FrameGraphResources {
    /// Get a resource based on its id
    pub fn get<'a>(
        &'a self,
        id: FrameGraphResourceId,
        frame_index: &FrameIndex,
    ) -> FrameGraphResource<'a> {
        if id.is_external() {
            (&self.external_resources[id.index()]).into()
        } else if id.is_transient_native_scale() {
            let (info, image_view, size) = self.native_scale.get(id.index(), **frame_index);
            FrameGraphResource::Transient {
                info,
                image_view,
                size,
            }
        } else {
            panic!("only external resources are currently supported")
        }
    }
}
