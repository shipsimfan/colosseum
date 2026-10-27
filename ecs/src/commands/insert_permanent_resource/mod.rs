use crate::Commands;

mod command;
mod new;

/// A command to insert a single permanent resource into the world
pub struct InsertPermanentResource<Resource: 'static> {
    /// The resource to insert
    resource: Resource,
}

impl Commands {
    /// Insert a permanent resource into the world
    pub fn insert_permanent_resource<Resource: 'static>(&mut self, resource: Resource) {
        self.queue(InsertPermanentResource::new(resource))
    }
}
