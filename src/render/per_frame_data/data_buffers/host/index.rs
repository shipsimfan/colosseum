use crate::render::{FrameHostDataBuffer, FrameIndex, HostDataBuffer};
use std::ops::{Index, IndexMut};

impl<T> Index<&FrameIndex> for HostDataBuffer<T> {
    type Output = FrameHostDataBuffer<T>;

    fn index(&self, index: &FrameIndex) -> &Self::Output {
        &self.frame[**index]
    }
}

impl<T> IndexMut<&FrameIndex> for HostDataBuffer<T> {
    fn index_mut(&mut self, index: &FrameIndex) -> &mut Self::Output {
        &mut self.frame[**index]
    }
}
