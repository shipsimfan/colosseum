use crate::SystemReturn;
use colosseum_core::Result;

impl SystemReturn for () {
    fn into_result(self) -> Result<()> {
        Ok(())
    }
}
