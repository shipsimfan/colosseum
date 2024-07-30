use crate::{RenderContext, UpdateContext};

/// A single coherent game state, such as a level or match
pub trait Scene: 'static {
    /// Update the game state by one frame
    fn update(&mut self, context: UpdateContext) -> Option<Box<dyn Scene>>;

    /// Render the current game state
    fn render(&mut self, context: RenderContext);

    /// Runs when the scene starts, before any calls to `update` or `render`
    #[allow(unused_variables)]
    fn on_start(&mut self, context: UpdateContext) {}

    /// Runs when the scene finishes, after all calls to `update` and `render`
    #[allow(unused_variables)]
    fn on_finish(&mut self, context: UpdateContext) {}
}
