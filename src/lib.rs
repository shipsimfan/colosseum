//! Cross-Platform, Multi-threaded Vulkan Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(incomplete_features)]
#![feature(generic_const_items)]
#![feature(const_trait_impl)]
#![feature(const_convert)]

pub mod file_io;
pub mod logging;
pub mod render;
pub mod settings;
pub mod update;

mod error;
mod game;
mod run;
mod threads;

pub use error::*;
pub use game::*;
pub use run::*;

pub(crate) use threads::*;

pub use alexandria::{Id, MemorySize, Uuid, input::KeyCode as Key, math};
