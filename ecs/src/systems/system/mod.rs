use crate::{UnsafeWorldCell, World, WorldAccess};
use colosseum_core::Result;
use std::any::TypeId;

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

    /// Get the access information for the system
    fn access(&self) -> &WorldAccess;

    /// Register a new archetype with the system
    fn register_archetype(&mut self, archetype: usize, components: &[TypeId]);

    /// Apply any deferred actions for the system
    #[allow(unused_variables)]
    fn apply(&mut self, world: &mut World) {}
}
