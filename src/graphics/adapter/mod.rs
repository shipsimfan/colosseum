use win32::{dxgi::IDXGIAdapter1, ComPtr};

mod output;

mod enumerate;
mod get;
mod new;

pub use output::{Output, OutputResolution};

/// A graphics adapter
pub struct Adapter {
    /// The name of the adapter
    name: String,

    /// The amount of dedicated video memory this adapter has
    video_memory: u64,

    /// The outputs this adapter can display to
    outputs: Vec<Output>,

    /// The underlying adapter
    #[allow(unused)]
    adapter: ComPtr<IDXGIAdapter1>,
}
