use crate::render::frame_graph::{FrameGraphNode, RenderScaleNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for RenderScaleNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Render Scale"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (1.0, 0.65, 0.20)
    }

    fn system(&self) -> impl 'static + System {
        RenderScaleNode::system.into_system()
    }
}
