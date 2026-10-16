use crate::ResMut;

impl<'a, Resource: 'static> ResMut<'a, Resource> {
    /// Create a new [`ResMut`] reference
    pub(crate) fn new(resource: &'a mut Resource) -> ResMut<'a, Resource> {
        ResMut { resource }
    }
}
