use colosseum_core::Result;

mod result;
mod unit;

/// A type that can be returned by a system
pub trait SystemReturn {
    /// Convert this type into a [`Result`]
    fn into_result(self) -> Result<()>;
}
