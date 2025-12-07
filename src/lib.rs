//! D3D11 Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(associated_type_defaults)]
#![feature(once_cell_try)]

use message_thread::MessageThread;
use run::RunningState;

pub mod graphics;
pub mod input;
pub mod logging;
pub mod math;
pub mod settings;
pub mod util;

mod error;
mod game;
mod managed_objects;
mod message_thread;
mod run;
mod scene;
mod update_context;

pub use error::{Error, Result};
pub use game::Game;
pub use managed_objects::*;
pub use run::*;
pub use scene::Scene;
pub use update_context::*;
