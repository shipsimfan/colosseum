//! The per-frame objects

mod acquire_image_semaphore;
mod draw_fence;

pub(in crate::render) use acquire_image_semaphore::*;
pub(in crate::render) use draw_fence::*;
