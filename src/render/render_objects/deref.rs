use crate::render::{FixedRenderObjects, RenderObjects};
use std::ops::Deref;

impl Deref for RenderObjects {
    type Target = FixedRenderObjects;

    fn deref(&self) -> &Self::Target {
        &self.fixed
    }
}
