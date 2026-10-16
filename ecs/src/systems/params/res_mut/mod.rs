mod deref;
mod new;
mod system_param;

/// An mutable reference to a resource in the ECS
pub struct ResMut<'a, Resource: 'static> {
    /// The mutable reference to the resource
    resource: &'a mut Resource,
}
