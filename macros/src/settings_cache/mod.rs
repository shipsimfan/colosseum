use attr::SettingsCacheAttr;
use output::SettingsCacheOutput;
use proc_macro_util::ast::Item;

mod attr;
mod output;

/// Produce a settings cache for the attached struct
pub fn settings_cache(
    item: Item,
    _: SettingsCacheAttr,
) -> proc_macro_util::Result<SettingsCacheOutput> {
    SettingsCacheOutput::from_input(item)
}
