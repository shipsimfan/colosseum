use crate::render::frame_graph::{FrameGraphExternalResource, FrameGraphResource};

impl<'a> From<&'a FrameGraphExternalResource> for FrameGraphResource<'a> {
    fn from(value: &'a FrameGraphExternalResource) -> Self {
        FrameGraphResource::External(value)
    }
}
