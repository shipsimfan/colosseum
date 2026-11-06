use crate::render::frame_graph::{FrameGraphNode, LitForwardRenderNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for LitForwardRenderNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Lit Forward Render"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (1.0, 1.0, 1.0)
    }

    fn system(&self) -> impl 'static + System {
        LitForwardRenderNode::system.into_system()
    }
}
