use attr::SettingsCacheAttr;
use output::SettingsCacheOutput;
use proc_macro_util::ast::items::Struct;

mod attr;
mod output;

/// Produce a settings cache for the attached struct
pub fn settings_cache(
    r#struct: Struct,
    _: SettingsCacheAttr,
) -> proc_macro_util::Result<SettingsCacheOutput> {
    SettingsCacheOutput::from_input(r#struct)
}
