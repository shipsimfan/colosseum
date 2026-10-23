use crate::{
    Result,
    render::{RenderGpuTransferQueue, RenderQueue},
};
use colosseum_ecs::ResMut;

impl RenderGpuTransferQueue {
    /// Process all pending transfer commands in the queue
    pub fn system(mut self: ResMut<Self>, mut queue: ResMut<RenderQueue>) -> Result<()> {
        while self.handle_command(&mut queue, false)? {}
        Ok(())
    }
}
