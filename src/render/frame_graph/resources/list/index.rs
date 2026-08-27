use crate::render::frame_graph::{FrameGraphTransientResource, resources::FrameGraphResourceList};
use std::ops::Index;

impl<'a> Index<usize> for FrameGraphResourceList<'a> {
    type Output = FrameGraphTransientResource;

    fn index(&self, index: usize) -> &Self::Output {
        &self.resources[index]
    }
}
