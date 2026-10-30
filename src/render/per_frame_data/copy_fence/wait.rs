use crate::{
    Result,
    render::{CopyFence, FrameIndex},
};
use colosseum_core::new_error;

impl CopyFence {
    /// Wait for the copy fence to be signaled
    pub fn wait(&mut self, frame_index: &FrameIndex) -> Result<()> {
        let fence = self.get_mut(frame_index);
        fence
            .wait(u64::MAX)
            .map(|_| ())
            .map_err(|error| new_error!("unable to wait on copy fence - {}", error))?;
        fence
            .reset()
            .map_err(|error| new_error!("unable to reset a copy fence - {}", error))
    }
}
