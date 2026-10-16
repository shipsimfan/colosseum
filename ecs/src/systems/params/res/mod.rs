mod deref;
mod new;
mod system_param;

/// An immutable reference to a resource in the ECS
pub struct Res<'a, Resource: 'static> {
    /// The immutable reference to the resource
    resource: &'a Resource,
}
