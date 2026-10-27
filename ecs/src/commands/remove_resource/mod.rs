use crate::Commands;
use std::marker::PhantomData;

mod command;
mod new;

/// A command to remove a single resource from the world
pub struct RemoveResource<Resource: 'static> {
    /// The type of the resource to remove
    _marker: PhantomData<fn() -> Resource>,
}

impl Commands {
    /// Remove a resource from the world
    pub fn remove_resource<Resource: 'static>(&mut self) {
        self.queue(RemoveResource::<Resource>::new())
    }
}
