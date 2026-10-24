use crate::update::{NextScene, Scene};

impl NextScene {
    /// Create a new [`NextScene`] container
    pub(in crate::update) fn new<InitialScene: Scene>(initial_scene: InitialScene) -> NextScene {
        NextScene {
            next_scene: Some(Box::new(initial_scene)),
        }
    }
}
