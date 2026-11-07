use crate::{
    Result,
    render::{DirectionalLight, ProceduralSkybox, Skybox, phases::PreRender},
};
use colosseum_ecs::{IntoSystem, System, UnsafeWorldCell, World, WorldAccess};

/// The system responsible for updating the procedural skybox
struct ProceduralSkyboxSystem {
    /// The index of the skybox resource
    skybox_index: usize,
}

impl ProceduralSkybox {
    /// Register the system for updating the procedural skybox
    pub(in crate::render) fn register_system(world: &mut World) {
        world.register_system(PreRender, ProceduralSkyboxSystem { skybox_index: 0 });
    }
}

impl System for ProceduralSkyboxSystem {
    fn initialize(&mut self, world: &mut World) {
        self.skybox_index = world.get_or_reserve_resource_index::<Skybox>();
    }

    fn access(&self) -> &WorldAccess {
        &WorldAccess::Exclusive
    }

    fn deferred(&self) -> bool {
        false
    }

    fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
        let mut skybox = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut::<Skybox>(self.skybox_index)
        };

        let skybox = match skybox.as_mut() {
            Skybox::Procedural(skybox) => skybox,
            _ => return Ok(()),
        };

        let directional_light = match skybox.directional_light {
            Some(directional_light) => directional_light,
            None => return Ok(()),
        };

        let directional_light =
            match unsafe { world.get() }.try_get::<DirectionalLight>(directional_light) {
                Some(directional_light) => directional_light,
                None => return Ok(()),
            };

        skybox.sun_direction = directional_light.direction();
        skybox.sun_color = directional_light.color();

        Ok(())
    }
}

impl IntoSystem<()> for ProceduralSkyboxSystem {
    type System = Self;

    fn into_system(self) -> Self::System {
        self
    }
}
