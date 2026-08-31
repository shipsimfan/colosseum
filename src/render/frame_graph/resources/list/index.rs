use crate::render::frame_graph::{FrameGraphTransientResource, resources::FrameGraphResourceList};
use std::ops::Index;

impl Index<usize> for FrameGraphResourceList {
    type Output = FrameGraphTransientResource;

    fn index(&self, index: usize) -> &Self::Output {
        &self.resources[index]
    }
}
