use std::marker::PhantomData;

/// An entity in the Entity Component System (ECS) system, referred to using an
/// [`Id<Entity>`](crate::Id)
pub struct Entity {
    /// A marker to prevent the struct from being constructed outside of the ECS system
    _marker: PhantomData<()>,
}
