use crate::render::frame_graph::{FrameGraphNode, UnlitForwardRenderNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for UnlitForwardRenderNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Unlit Forward Render"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (0.30, 0.90, 0.45)
    }

    fn system(&self) -> impl 'static + System {
        UnlitForwardRenderNode::system.into_system()
    }
}
