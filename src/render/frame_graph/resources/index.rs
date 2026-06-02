use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};
use std::ops::Index;

impl<'a> Index<FrameGraphResourceId> for FrameGraphResources<'a> {
    type Output = FrameGraphResource<'a>;

    fn index(&self, index: FrameGraphResourceId) -> &Self::Output {
        self.get(index).expect("resource ID is invalid")
    }
}
