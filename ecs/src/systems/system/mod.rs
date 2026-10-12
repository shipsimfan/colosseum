use crate::{Archetypes, Resources};
use colosseum_core::Result;

mod function;
mod into;
mod r#return;

pub use function::*;
pub use into::*;
pub use r#return::*;

/// A single system in the ECS
pub trait System {
    /// Execute the system
    fn run(&mut self, resources: &mut Resources, archetypes: &mut Archetypes) -> Result<()>;
}
