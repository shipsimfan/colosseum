//! The graphics subsystem

mod adapter;
mod context;
mod settings;

pub use adapter::{Adapter, Output, OutputResolution};
pub use settings::{DisplayMode, GraphicsSettings};

pub(crate) use context::GraphicsContext;
