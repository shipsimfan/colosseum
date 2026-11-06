use crate::render::{
    ShadowMapLight,
    frame_graph::{FrameGraphNode, ShadowMapNode},
};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl<L: ShadowMapLight> FrameGraphNode for ShadowMapNode<L> {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        format!("{} Shadow Map", L::NAME)
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        L::COLOR
    }

    fn system(&self) -> impl 'static + System {
        ShadowMapNode::<L>::system.into_system()
    }
}
