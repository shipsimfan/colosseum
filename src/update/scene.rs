use crate::Result;
use colosseum_ecs::{SystemReturn, World};

/// A single scene of a game
pub trait Scene: 'static + Send {
    /// Start a scene
    fn start(self, world: &mut World) -> Result<()>;
}

impl<R: SystemReturn, F: 'static + Send + FnOnce(&mut World) -> R> Scene for F {
    fn start(self, world: &mut World) -> Result<()> {
        (self)(world).into_result()
    }
}
