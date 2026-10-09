//! Cross-Platform, Multi-threaded Vulkan Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(incomplete_features)]
/*
#![feature(generic_const_items)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
*/

pub mod update;

mod file_io;
mod game;
mod options;
mod run;
mod settings;

pub use file_io::*;
pub use game::*;
pub use options::*;
pub use run::*;
pub use settings::*;

pub use colosseum_core::{
    Error, GameMetadata, LogSeverity, Logger, LoggingOptions, Result, SingleValueReceiver,
    SingleValueSender, debug, debug_s, error, error_s, info, info_s, log, log_s, logger,
    single_value_channel, warning, warning_s,
};

pub use alexandria::math;

/*
pub mod render;

pub use alexandria::{Id, MemorySize, Uuid, input::KeyCode as Key, math};
*/
