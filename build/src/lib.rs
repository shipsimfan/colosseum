//! Utilities to help produce build.rs for games

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod branch;
mod build_time;
mod commit_hash;

pub use branch::*;
pub use build_time::*;
pub use commit_hash::*;
