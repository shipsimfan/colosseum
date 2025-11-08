//! D3D11 Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod graphics;
pub mod logging;
pub mod math;
pub mod settings;
pub mod util;

mod error;
mod game;
mod macros;
mod run;
mod scene;

pub use error::{Error, Result};
pub use game::Game;
pub use run::*;
pub use scene::Scene;
