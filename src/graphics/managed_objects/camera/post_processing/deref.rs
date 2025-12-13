use crate::{
    graphics::{CameraPostProcessing, PostProcessingShader},
    util::Arena,
};
use std::ops::{Deref, DerefMut};

impl Deref for CameraPostProcessing {
    type Target = Arena<PostProcessingShader>;

    fn deref(&self) -> &Self::Target {
        &self.provided_post_processing
    }
}

impl DerefMut for CameraPostProcessing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.provided_post_processing
    }
}
