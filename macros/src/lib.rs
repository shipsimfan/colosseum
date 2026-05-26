//! Procedural macros for colosseum

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(proc_macro_diagnostic)]

mod settings_cache;

proc_macro_util::proc_macro_attribute!(
    /// Converts a struct into a settings cache
    settings_cache -> settings_cache::settings_cache
);
