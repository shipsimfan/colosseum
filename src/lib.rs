//! A game engine built on Alexandria

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(negative_impls)]

pub mod logging;

mod run;
mod scene;
mod settings;
mod state;

pub use alexandria::Error;
pub use run::run;
pub use scene::Scene;
pub use settings::{LoadSettingsError, SaveSettingsError, Settings, SettingsController};
pub use state::{RenderContext, UpdateContext};

/// Is the program running in debug-mode?
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;

/// Is the program running in debug-mode?
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;
