use crate::InsertResource;

impl<Resource: 'static> InsertResource<Resource> {
    /// Create a new [`InsertResource`] command
    pub fn new(resource: Resource) -> InsertResource<Resource> {
        InsertResource { resource }
    }
}
