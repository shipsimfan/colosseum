use crate::render::{FrameGraph, GraphicsDevice};
use std::{marker::PhantomData, sync::Arc};

impl<Game: crate::Game> FrameGraph<Game> {
    /// Create a new [`FrameGraph`]
    pub fn new(device: &Arc<GraphicsDevice>) -> FrameGraph<Game> {
        FrameGraph {
            structure: None,
            device: device.clone(),
            game: PhantomData,
        }
    }
}
