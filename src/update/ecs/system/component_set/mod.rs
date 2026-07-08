use crate::update::ecs::Archetype;
use std::any::TypeId;

mod tuple;

/// A set of components that a system operates on in the ECS framework
pub trait ComponentSet<'a> {
    /// The number of components in the set
    const COUNT: usize;

    /// The type IDs of the contained components, in order
    const TYPE_IDS: &'static [TypeId];

    /// Create a new [`ComponentSet`] from an [`Archetype`]
    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self;
}
