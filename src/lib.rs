//! Cross-Platform, Multi-threaded Vulkan Game Engine

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(const_convert)]
#![feature(const_trait_impl)]

pub mod render;
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
    SingleValueSender, debug, debug_s, error, error_s, info, info_s, log, log_s, logger, new_error,
    single_value_channel, warning, warning_s,
};
pub use colosseum_ecs as ecs;

pub use alexandria::{Id, MemorySize, Uuid, input::KeyCode as Key, math};
