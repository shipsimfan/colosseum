use constant_buffer::LightConstantBuffer;

mod constant_buffer;

mod bind;
mod get;
mod new;
mod set;

/// The lights registered with the engine
pub(in crate::graphics) struct Lights {
    /// The global information about lighting
    constant_buffer: LightConstantBuffer,
}
