use crate::Result;
use colosseum_ecs::World;

/// The main render phase
pub(in crate::render) struct Render;

/// Execute all render-related phases
pub(crate) fn execute_render_systems(world: &mut World) -> Result<()> {
    world.run::<Render>()
}
