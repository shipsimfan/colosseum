use crate::update::ecs::Archetype;

mod new;

/// The set of archetypes in the Entity Component System (ECS) system
pub(in crate::update::ecs) struct ArchetypeSet {
    /// The actual archetypes in the ECS system
    archetypes: Vec<Archetype>,
}
