use colosseum_core::Result;

/// A type that can be returned by a system
pub trait SystemReturn {
    /// Convert this type into a [`Result`]
    fn into_result(self) -> Result<()>;
}

impl SystemReturn for Result<()> {
    fn into_result(self) -> Result<()> {
        self
    }
}

impl SystemReturn for () {
    fn into_result(self) -> Result<()> {
        Ok(())
    }
}
