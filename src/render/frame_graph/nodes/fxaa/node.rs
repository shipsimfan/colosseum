use crate::render::frame_graph::{FrameGraphNode, FxaaNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for FxaaNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "FXAA"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (1.0, 0.45, 0.30)
    }

    fn system(&self) -> impl 'static + System {
        FxaaNode::system.into_system()
    }
}
