mod add_component;
mod add_components;
mod clear_phase;
mod command;
mod create_entity;
mod insert_permanent_resource;
mod insert_resource;
mod register_system;
mod remove_component;
mod remove_entity;
mod remove_resource;
mod spawn_entity;

mod queue;
mod system_param;

pub use add_component::*;
pub use add_components::*;
pub use clear_phase::*;
pub use command::*;
pub use create_entity::*;
pub use insert_permanent_resource::*;
pub use insert_resource::*;
pub use register_system::*;
pub use remove_component::*;
pub use remove_entity::*;
pub use remove_resource::*;
pub use spawn_entity::*;

/// Deferred commands to be executed at the end of the phase
pub struct Commands {
    /// The list of deferred commands to be executed at the end of the phase
    commands: Vec<Box<dyn Command>>,
}
