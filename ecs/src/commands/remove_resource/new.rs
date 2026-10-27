use crate::RemoveResource;
use std::marker::PhantomData;

impl<Resource: 'static> RemoveResource<Resource> {
    /// Create a new [`RemoveResource`] command
    pub fn new() -> RemoveResource<Resource> {
        RemoveResource {
            _marker: PhantomData,
        }
    }
}
