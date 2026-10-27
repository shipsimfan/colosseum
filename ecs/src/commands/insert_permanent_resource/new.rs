use crate::InsertPermanentResource;

impl<Resource: 'static> InsertPermanentResource<Resource> {
    /// Create a new [`InsertPermanentResource`] command
    pub fn new(resource: Resource) -> InsertPermanentResource<Resource> {
        InsertPermanentResource { resource }
    }
}
