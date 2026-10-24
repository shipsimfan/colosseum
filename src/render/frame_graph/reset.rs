use crate::render::FrameGraph;

impl<Game: crate::Game> FrameGraph<Game> {
    /// Reset the frame graph to its initial state
    pub fn reset(&mut self) {
        self.structure = None;
    }
}
