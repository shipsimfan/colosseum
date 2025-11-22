//! Utilities to help produce build.rs for games

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod build_time;
mod commit_hash;

pub use build_time::{get_build_time, push_build_time, push_game_build_time};
pub use commit_hash::{get_commit_hash, push_commit_hash, push_game_commit_hash};
