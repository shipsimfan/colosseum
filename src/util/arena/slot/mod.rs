mod free;
mod get;
mod new;
mod set;

/// A slot in an arena that may contain a value
pub(crate) struct Slot<T: Sized> {
    /// The item contained in this slot
    item: Option<T>,

    /// The generation of the slot
    generation: u32,

    /// The next free slot, or u32::MAX if there is none
    next_free: u32,
}
