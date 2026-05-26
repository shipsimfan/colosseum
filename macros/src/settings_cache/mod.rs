use attr::SettingsCacheAttr;
use proc_macro_util::{Result, ast::Item};

mod attr;
mod input;
mod output;

pub use input::*;
pub use output::*;

/// Produce a settings cache for the attached struct
pub fn settings_cache(item: Item, _: SettingsCacheAttr) -> Result<SettingsCacheOutput> {
    let input = SettingsCacheInput::new(item)?;

    Ok(SettingsCacheOutput::from_input(input))
}
