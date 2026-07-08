use crate::update::ecs::Archetype;
use std::any::TypeId;

//mod component_set;

mod execute;
mod new;
mod register_archetype;

//pub use component_set::*;

/// A system in the ECS framework, which operates on a set of components and performs some logic on
/// them
pub struct System {
    /// The type IDs of the components that the system operates on, in order
    component_type_ids: Vec<TypeId>,

    /// The indices of the components in the archetypes that the system operates on, along with the
    /// archetype index
    ///
    /// The elements are stored inline, with the first element of a set being the archetype index,
    /// and the rest being the indices of the components in that archetype
    indices: Vec<usize>,

    /// The system itself
    system: Box<dyn FnMut(&mut [Archetype], &[usize])>,
}
