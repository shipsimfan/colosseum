//! ECS system for Colosseum

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod archetypes;
mod commands;
mod resources;
mod systems;
mod world;

pub use archetypes::*;
pub use commands::*;
pub use systems::*;
pub use world::*;

pub(crate) use resources::*;
