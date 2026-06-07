use crate::settings_cache::{
    SettingsCacheInput, SettingsCacheOutput, SettingsCacheOutputGetterFns,
    SettingsCacheOutputModifiableGetterFns, SettingsCacheOutputModifiableStruct,
    SettingsCacheOutputSetterFns, SettingsCacheOutputStruct, SettingsCacheOutputTrait,
};

impl<'a> SettingsCacheOutput<'a> {
    /// Create a new [`SettingsCacheOutput`] from a [`SettingsCacheInput`]
    pub fn from_input(input: SettingsCacheInput<'a>) -> SettingsCacheOutput<'a> {
        SettingsCacheOutput {
            r#trait: SettingsCacheOutputTrait::from_input(&input),
            getters: SettingsCacheOutputGetterFns::from_input(input.name.clone(), &input.fields),
            modifiable_struct: SettingsCacheOutputModifiableStruct::from_input(&input),
            modifiable_getters: SettingsCacheOutputModifiableGetterFns::from_input(
                input.modifiable_name.clone(),
                &input.fields,
            ),
            setters: SettingsCacheOutputSetterFns::from_input(&input),
            r#struct: SettingsCacheOutputStruct::from_input(input),
        }
    }
}
