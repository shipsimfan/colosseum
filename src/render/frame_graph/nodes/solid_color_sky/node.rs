use crate::render::frame_graph::{FrameGraphNode, SolidColorSkyNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for SolidColorSkyNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Solid Color Sky"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (0.65, 0.85, 1.0)
    }

    fn system(&self) -> impl 'static + System {
        SolidColorSkyNode::system.into_system()
    }
}
