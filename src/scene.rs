/// A single coherent game state, such as a level or match
pub trait Scene: 'static {
    /// Update the game state by one frame
    fn update(&mut self) -> Option<Box<dyn Scene>>;

    /// Render the current game state
    fn render(&mut self);
}
