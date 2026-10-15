use crate::{Result, render::GraphicsDevice};
use colosseum_core::new_error;
use colosseum_ecs::World;

/// Stop any ongoing rendering operations for the given world for clean shutdown
pub(crate) fn stop_render(world: &mut World) -> Result<()> {
    world
        .get_resource::<GraphicsDevice>()
        .wait_idle()
        .map_err(|error| new_error!("unable to wait for the graphics device to idle - {}", error))
}
