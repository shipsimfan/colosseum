use crate::render::{Camera, FrameIndex, GpuCamera, HostDataBuffer, Swapchain};
use colosseum_ecs::{Res, ResMut};

impl Camera {
    /// Update the view-projection of the camera
    pub(in crate::render) fn system(
        self: ResMut<Camera>,
        swapchain: Res<Swapchain>,
        mut buffer: ResMut<HostDataBuffer<GpuCamera>>,
        frame_index: Res<FrameIndex>,
    ) {
        let size = swapchain.size();
        let gpu_camera = GpuCamera::new(
            self.position,
            self.rotation,
            &self.projection,
            size.x as f32 / size.y as f32,
        );

        buffer[frame_index.as_ref()][0] = gpu_camera;
    }
}
