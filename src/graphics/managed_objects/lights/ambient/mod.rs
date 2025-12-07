use crate::graphics::util::ConstantBuffer;
use cb_content::LightCbContent;

mod cb_content;

mod bind;
mod get;
mod new;
mod set;

/// The lights registered with the engine
pub struct AmbientLight {
    /// The global information about lighting
    constant_buffer: ConstantBuffer<LightCbContent>,
}
