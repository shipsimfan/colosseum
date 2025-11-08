use crate::graphics::{Adapter, Output};
use std::slice::SliceIndex;
use win32::dxgi::IDXGIAdapter1;

impl Adapter {
    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the dedicated video memory, in bytes, this adapter has
    pub const fn video_memory(&self) -> u64 {
        self.video_memory
    }

    /// Get the outputs this adapter can display to
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    /// Get the output at index `index`
    pub fn output<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[Output]>,
    {
        self.outputs.get::<I>(index)
    }

    /// Get the number of outputs this output can display on
    pub const fn num_outputs(&self) -> usize {
        self.outputs.len()
    }

    /// Get the underlying adapter handle
    pub(crate) fn handle(&mut self) -> &mut IDXGIAdapter1 {
        &mut self.adapter
    }
}
