use crate::math::Vector2i;
use win32::{dxgi::IDXGIAdapter1, dxgi1_2::IDXGIOutput1, ComPtr};

mod resolution;

mod enumerate;
mod get;
mod new;

pub use resolution::OutputResolution;

/// An output which an adapter can display to
pub struct Output {
    /// The name of the output
    name: String,

    /// The position of the upper-left hand corner of the output among all outputs
    position: Vector2i,

    /// The resolutions this output supports
    resolutions: Vec<OutputResolution>,

    /// The underlying output
    #[allow(unused)]
    output: ComPtr<IDXGIOutput1>,

    /// The adapter this output comes from
    #[allow(unused)]
    adapter: ComPtr<IDXGIAdapter1>,
}
