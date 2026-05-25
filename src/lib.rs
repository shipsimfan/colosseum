//! Cross-Platform, Multi-threaded Vulkan Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(box_into_inner)]

pub mod logging;
pub mod settings;

mod error;
mod game;
mod run;
mod threads;

pub use error::*;
pub use game::*;
pub use run::*;

pub(crate) use threads::*;
