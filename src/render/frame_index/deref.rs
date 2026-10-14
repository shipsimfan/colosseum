use crate::render::FrameIndex;
use std::ops::Deref;

impl Deref for FrameIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}
