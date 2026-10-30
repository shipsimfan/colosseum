use crate::{
    Window,
    render::{
        CopyFence, DrawFence, FrameGraph, FrameIndex, GpuDataBuffers, RenderIndices, Swapchain,
    },
    update::NextScene,
};
use colosseum_ecs::{Res, ResMut, World};

impl<Game: crate::Game> RenderIndices<Game> {
    /// Get a reference to the window resource
    pub fn window<'a>(&self, world: &'a World) -> Res<'a, Window> {
        unsafe { world.get_resource_at_index(self.window) }
    }

    /// Get a reference to the settings resource
    pub fn settings<'a>(&self, world: &'a World) -> Res<'a, Game::SettingsCache> {
        unsafe { world.get_resource_at_index(self.settings) }
    }

    /// Get a mutable reference to the next scene resource
    pub fn next_scene_mut<'a>(&self, world: &'a mut World) -> ResMut<'a, NextScene> {
        unsafe { world.get_resource_at_index_mut(self.next_scene) }
    }

    /// Get a reference to the swapchain resource
    pub(in crate::render) fn swapchain<'a>(&self, world: &'a World) -> Res<'a, Swapchain> {
        unsafe { world.get_resource_at_index(self.swapchain) }
    }

    /// Try to get a reference to the swapchain resource
    pub(in crate::render) fn try_swapchain<'a>(
        &self,
        world: &'a World,
    ) -> Option<Res<'a, Swapchain>> {
        unsafe { world.try_get_resource_at_index(self.swapchain) }
    }

    /// Get a reference to the frame index resource
    pub(in crate::render) fn frame_index<'a>(&self, world: &'a World) -> Res<'a, FrameIndex> {
        unsafe { world.get_resource_at_index(self.frame_index) }
    }

    /// Get a mutable reference to the frame index resource
    pub(in crate::render) fn frame_index_mut<'a>(
        &self,
        world: &'a mut World,
    ) -> ResMut<'a, FrameIndex> {
        unsafe { world.get_resource_at_index_mut(self.frame_index) }
    }

    /// Get a mutable reference to the frame graph resource
    pub(in crate::render) fn frame_graph_mut<'a>(
        &self,
        world: &'a mut World,
    ) -> ResMut<'a, FrameGraph<Game>> {
        unsafe { world.get_resource_at_index_mut(self.frame_graph) }
    }

    /// Get a mutable reference to the draw fence resource
    pub(in crate::render) fn draw_fence_mut<'a>(
        &self,
        world: &'a mut World,
    ) -> ResMut<'a, DrawFence> {
        unsafe { world.get_resource_at_index_mut(self.draw_fence) }
    }

    /// Get a mutable reference to the copy fence resource
    pub(in crate::render) fn copy_fence_mut<'a>(
        &self,
        world: &'a mut World,
    ) -> ResMut<'a, CopyFence> {
        unsafe { world.get_resource_at_index_mut(self.copy_fence) }
    }

    /// Get a reference to the GPU data buffers resource
    pub(in crate::render) fn gpu_data_buffers<'a>(
        &self,
        world: &'a World,
    ) -> Res<'a, GpuDataBuffers> {
        unsafe { world.get_resource_at_index(self.gpu_data_buffers) }
    }
}
