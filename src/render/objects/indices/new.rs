use crate::{
    Window,
    render::{
        CopyFence, DrawFence, FrameGraph, FrameIndex, GpuDataBuffers, RenderIndices, Swapchain,
    },
    update::NextScene,
};
use colosseum_ecs::World;
use std::marker::PhantomData;

impl<Game: crate::Game> RenderIndices<Game> {
    /// Create a new set of [`RenderIndices`]
    pub(in crate::render) fn new(world: &mut World) -> RenderIndices<Game> {
        let settings = world.get_or_reserve_permanent_resource_index::<Game::SettingsCache>();
        let window = world.get_or_reserve_permanent_resource_index::<Window>();
        let next_scene = world.get_or_reserve_permanent_resource_index::<NextScene>();

        let swapchain = world.get_or_reserve_permanent_resource_index::<Swapchain>();
        let frame_index = world.get_or_reserve_permanent_resource_index::<FrameIndex>();
        let frame_graph = world.get_or_reserve_permanent_resource_index::<FrameGraph<Game>>();
        let draw_fence = world.get_or_reserve_permanent_resource_index::<DrawFence>();
        let copy_fence = world.get_or_reserve_permanent_resource_index::<CopyFence>();
        let gpu_data_buffers = world.get_or_reserve_permanent_resource_index::<GpuDataBuffers>();

        RenderIndices {
            window,
            settings,
            next_scene,

            swapchain,
            frame_index,
            frame_graph,
            draw_fence,
            copy_fence,
            gpu_data_buffers,

            _marker: PhantomData,
        }
    }
}
