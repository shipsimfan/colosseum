use crate::{
    Window,
    render::{Camera, GpuCamera, GpuDataBuffer},
};
use colosseum_ecs::{Res, ResMut};

impl Camera {
    /// Update the view-projection of the camera
    pub(in crate::render) fn system(
        self: ResMut<Camera>,
        window: Res<Window>,
        buffer: ResMut<GpuDataBuffer<GpuCamera>>,
    ) {
        todo!()
    }
}
