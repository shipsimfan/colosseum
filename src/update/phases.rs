use crate::Result;
use colosseum_ecs::{PhaseName, World};

/// Runs before the main update phase of the game loop
#[derive(PartialEq, Eq)]
pub struct PreUpdate;

impl PhaseName for PreUpdate {}

/// The main update phase of the game loop
#[derive(PartialEq, Eq)]
pub struct Update;

impl PhaseName for Update {}

/// Runs after the main update phase of the game loop
#[derive(PartialEq, Eq)]
pub struct PostUpdate;

impl PhaseName for PostUpdate {}

/// Runs when the scene is about to be transitioned away from
#[derive(PartialEq, Eq)]
pub struct SceneTransition;

impl PhaseName for SceneTransition {}

/// Execute all the update-related phases
pub(crate) fn execute_update_systems(world: &mut World) -> Result<()> {
    world.run(PreUpdate)?;
    world.run(Update)?;
    world.run(PostUpdate)
}
