use crate::{Game, Scene, logging::LogController};
use std::sync::Arc;

/// A scene which can be used as the starting scene for a game
pub trait InitialScene: Scene {
    /// Create the new scene using `options`
    fn new(options: &<Self::Game as Game>::Options, log_controller: &Arc<LogController>) -> Self;
}
