use proc_macro_util::{Parse, Parser, Result};

pub struct SettingsCacheAttr;

impl<'a> Parse<'a> for SettingsCacheAttr {
    fn parse(_: &mut Parser<'a>) -> Result<Self> {
        Ok(SettingsCacheAttr)
    }
}
