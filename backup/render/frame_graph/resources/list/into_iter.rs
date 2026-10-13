use crate::render::frame_graph::{FrameGraphTransientResource, resources::FrameGraphResourceList};

impl<'a> IntoIterator for &'a FrameGraphResourceList {
    type Item = &'a FrameGraphTransientResource;
    type IntoIter = std::slice::Iter<'a, FrameGraphTransientResource>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.iter()
    }
}

impl<'a> IntoIterator for &'a mut FrameGraphResourceList {
    type Item = &'a mut FrameGraphTransientResource;
    type IntoIter = std::slice::IterMut<'a, FrameGraphTransientResource>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.iter_mut()
    }
}
