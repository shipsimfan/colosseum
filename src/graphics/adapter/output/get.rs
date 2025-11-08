use crate::{
    graphics::{Output, OutputResolution},
    math::Vector2i,
};
use std::slice::SliceIndex;

impl Output {
    /// Gets the name of the output
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the position of the upper-left hand corner of the output relative to all outputs
    pub const fn position(&self) -> Vector2i {
        self.position
    }

    /// Gets the x position of the upper-left hand corner of the output
    pub const fn x(&self) -> i32 {
        self.position.x
    }

    /// Gets the y position of the upper-left hand corner of the output
    pub const fn y(&self) -> i32 {
        self.position.y
    }

    /// Get the resolutions this output has available
    pub fn resolutions(&self) -> &[OutputResolution] {
        &self.resolutions
    }

    /// Get the resolution at index `index`
    pub fn resolution<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[OutputResolution]>,
    {
        self.resolutions.get::<I>(index)
    }

    /// Get the number of resolutions this output has available
    pub const fn num_resolutions(&self) -> usize {
        self.resolutions.len()
    }
}
