use crate::SystemReturn;
use colosseum_core::Result;

impl SystemReturn for Result<()> {
    fn into_result(self) -> Result<()> {
        self
    }
}
