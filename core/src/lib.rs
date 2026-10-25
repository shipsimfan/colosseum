//! Core definitions for Colosseum

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod as_bytes;
mod dyn_eq;
mod error;
mod game_metadata;
mod logging;
mod single_value_channel;
mod threads;

pub use as_bytes::*;
pub use dyn_eq::*;
pub use error::*;
pub use game_metadata::*;
pub use logging::*;
pub use single_value_channel::*;
pub use threads::*;
