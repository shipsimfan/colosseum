use crate::{Game, Scene, UpdateContext};

/// A scene which can be used as the starting scene for a game
pub trait InitialScene: Scene {
    /// Create the new scene using `options`
    fn new(
        options: &<Self::Game as Game>::Options,
        context: &mut UpdateContext<Self::Game>,
    ) -> Self;
}
