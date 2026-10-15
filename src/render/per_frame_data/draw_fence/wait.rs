use crate::{
    Result,
    render::{DrawFence, FrameIndex},
};
use colosseum_core::new_error;

impl DrawFence {
    /// Wait for the draw fence to be signaled
    pub fn wait(&mut self, frame_index: &FrameIndex) -> Result<()> {
        let (fence, submitted) = &mut self.fences[**frame_index];
        if *submitted {
            fence
                .wait(u64::MAX)
                .map(|_| ())
                .map_err(|error| new_error!("unable to wait on draw fence - {}", error))?;
            fence
                .reset()
                .map_err(|error| new_error!("unable to reset a draw fence - {}", error))?;
        }

        *submitted = false;
        Ok(())
    }
}
