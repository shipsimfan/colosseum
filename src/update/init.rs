use crate::update::{
    DeltaTime, Input, NextScene, PostUpdate, PreUpdate, Scene, Update, settings_update_system,
};
use colosseum_ecs::World;

/// Initialize all the resources required for updating
pub(crate) fn init_update_resources<Game: crate::Game, InitialScene: Scene>(
    world: &mut World,
    initial_scene: InitialScene,
) {
    world.insert_permanent_resource(DeltaTime::new());
    world.insert_permanent_resource(Input::new());
    world.insert_permanent_resource(NextScene::new(initial_scene));
}

/// Initialize the all the required update systems
pub(crate) fn init_update_systems<Game: crate::Game>(world: &mut World) {
    init_pre_update::<Game>(world);
    init_update(world);
    init_post_update(world);
}

fn init_pre_update<Game: crate::Game>(world: &mut World) {
    world.register_system(PreUpdate, DeltaTime::system);
    world.register_system(PreUpdate, settings_update_system::<Game>);
    world.register_system(PreUpdate, Input::system);
}

fn init_update(world: &mut World) {
    world.register_phase(Update);
}

fn init_post_update(world: &mut World) {
    world.register_phase(PostUpdate);
}
