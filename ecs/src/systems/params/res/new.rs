use crate::Res;

impl<'a, Resource: 'static> Res<'a, Resource> {
    /// Create a new [`Res`] reference
    pub(crate) fn new(resource: &'a Resource) -> Res<'a, Resource> {
        Res { resource }
    }
}
