use crate::Commands;

mod command;
mod new;

/// A command to insert a single resource into the world
pub struct InsertResource<Resource: 'static> {
    /// The resource to insert
    resource: Resource,
}

impl Commands {
    /// Insert a resource into the world
    pub fn insert_resource<Resource: 'static>(&mut self, resource: Resource) {
        self.queue(InsertResource::new(resource))
    }
}
