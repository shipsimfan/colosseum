use crate::update::Scene;

mod new;
mod set;
mod take;

/// A container for the next scene to be transitioned to
pub struct NextScene {
    /// The function to initialize the next scene
    next_scene: Option<Box<dyn Scene>>,
}
