use alexandria::{
    Notify,
    math::{Vector2i, Vector2u},
};
use std::sync::atomic::{AtomicBool, AtomicU64};

mod get;
mod new;
mod set;

/// The state of the window directly shared between the game thread and the WSI thread
pub(in crate::run) struct SharedWindow {
    /// The position of the window, encoded as a single `u64` with the x position in the upper 32
    /// bits and the y position in the lower 32 bits.
    position: AtomicU64,

    /// The size of the window, encoded as a single `u64` with the width in the upper 32 bits and
    /// the height in the lower 32 bits. If the window is minimized, this will be set to `0`.
    size: AtomicU64,

    /// Whether the window is currently fullscreen or not
    fullscreen: AtomicBool,

    /// Whether the window is currently maximized or not
    maximized: AtomicBool,

    /// A notify for when the window has been restored from a minimized state
    restored_notify: Notify,
}

/// Encode a [`Vector2i`] as a single `u64` with the x position in the upper 32 bits and the y
/// position in the lower 32 bits
fn encode_position(position: Vector2i) -> u64 {
    (position.x as u64) << 32 | (position.y as u64)
}

/// Decode a `u64` encoded by [`encode_position`] back into a [`Vector2i`]
fn decode_position(encoded: u64) -> Vector2i {
    Vector2i {
        x: (encoded >> 32) as i32,
        y: (encoded & 0xFFFFFFFF) as i32,
    }
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
