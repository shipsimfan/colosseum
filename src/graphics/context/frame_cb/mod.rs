mod new;
mod update;

/// Per frame data for shaders
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::context) struct FrameCb {
    /// The current frame
    frame: u32,

    /// The current time since startup, in seconds
    time: f32,

    /// The difference in time since last frame, in seconds
    delta_time: f32,

    /// A reserved value
    reserved: u32,
}
