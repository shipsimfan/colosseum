use alexandria::math::Vector2u;
use std::sync::atomic::AtomicU64;

mod get;
mod new;
mod set;

/// The state of the window directly shared between the game thread and the WSI thread
pub(in crate::run::wsi) struct SharedWindow {
    /// The size of the window, encoded as a single `u64` with the width in the upper 32 bits and
    /// the height in the lower 32 bits
    size: AtomicU64,
}

/// Encodes a [`Vector2u`] as a single `u64` with the width in the upper 32 bits and the height in
/// the lower 32 bits
fn encode_size(size: Vector2u) -> u64 {
    (size.x as u64) << 32 | (size.y as u64)
}

/// Decodes a `u64` encoded by [`encode_size`] back into a [`Vector2u`]
fn decode_size(encoded: u64) -> Vector2u {
    Vector2u {
        x: (encoded >> 32) as u32,
        y: (encoded & 0xFFFFFFFF) as u32,
    }
}
