use crate::Result;

/// A single scene of a game
pub trait Scene: 'static {
    /// Start a scene
    fn start(self) -> Result<()>;
}

impl<F: 'static + FnOnce() -> Result<()>> Scene for F {
    fn start(self) -> Result<()> {
        (self)()
    }
}
