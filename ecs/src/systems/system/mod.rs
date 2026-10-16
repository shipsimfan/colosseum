use crate::{UnsafeWorldCell, World};
use colosseum_core::Result;

mod function;
mod into;
mod r#return;

pub use function::*;
pub use into::*;
pub use r#return::*;

/// A single system in the ECS
pub trait System {
    /// Initialize the system parameters
    fn initialize(&mut self, world: &mut World);

    /// Execute the system
    fn run(&mut self, world: &UnsafeWorldCell) -> Result<()>;

    /// Does the system have deferred actions?
    fn deferred(&self) -> bool {
        false
    }

    /// Apply any deferred actions for the system
    #[allow(unused_variables)]
    fn apply(&mut self, world: &mut World) {}
}
