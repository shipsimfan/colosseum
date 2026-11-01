use crate::render::frame_graph::{FrameGraphNode, ProceduralSkyNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for ProceduralSkyNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Procedural Sky"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (0.4, 0.7, 1.0)
    }

    fn system(&self) -> impl 'static + System {
        ProceduralSkyNode::system.into_system()
    }
}
