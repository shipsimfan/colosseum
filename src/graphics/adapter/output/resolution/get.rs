use crate::{
    graphics::OutputResolution,
    math::{Rational, Vector2u},
};
use std::slice::SliceIndex;

impl OutputResolution {
    /// Get the size of this resolution, in pixels
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the width of this resolution, in pixels
    pub const fn width(&self) -> u32 {
        self.size.x
    }

    /// Get the height of this resolution, in pixels
    pub const fn height(&self) -> u32 {
        self.size.y
    }

    /// Get the refresh rates this resolution has available
    pub fn refresh_rates(&self) -> &[Rational<u32>] {
        &self.refresh_rates
    }

    /// Get the refresh rate at index `index`
    pub fn refresh_rate<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[Rational<u32>]>,
    {
        self.refresh_rates.get::<I>(index)
    }

    /// Get the number of refresh rates this resolution has available
    pub const fn num_refresh_rates(&self) -> usize {
        self.refresh_rates.len()
    }
}
