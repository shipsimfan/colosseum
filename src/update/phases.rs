use crate::Result;
use colosseum_ecs::World;

/// Runs before the main update phase of the game loop
pub struct PreUpdate;

/// The main update phase of the game loop
pub struct Update;

/// Runs after the main update phase of the game loop
pub struct PostUpdate;

/// Copies the render-related data into render-specific structures
pub(in crate::update) struct PreRender;

/// Execute all the update-related phases
pub(crate) fn execute_update_systems(world: &mut World) -> Result<()> {
    world.run::<PreUpdate>()?;
    world.run::<Update>()?;
    world.run::<PostUpdate>()?;
    world.run::<PreRender>()
}
