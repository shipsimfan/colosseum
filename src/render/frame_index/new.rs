use crate::render::{FrameIndex, MAX_FRAMES_IN_FLIGHT, PostRender};
use colosseum_ecs::{ResMut, World};

impl FrameIndex {
    /// Create a new [`FrameIndex`] starting at 0
    pub fn new(world: &mut World) {
        world.insert_resource(FrameIndex { index: 0 });

        world.register_system::<PostRender, _, _>(|mut frame_index: ResMut<FrameIndex>| {
            frame_index.index = (frame_index.index + 1) % MAX_FRAMES_IN_FLIGHT;
        });
    }
}
