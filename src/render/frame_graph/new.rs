use crate::render::FrameGraph;
use std::marker::PhantomData;

impl<Game: crate::Game> FrameGraph<Game> {
    /// Create a new [`FrameGraph`]
    pub fn new() -> FrameGraph<Game> {
        FrameGraph {
            structure: None,
            game: PhantomData,
        }
    }
}
