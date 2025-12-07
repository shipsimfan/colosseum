use crate::util::Handle;

impl<T: Sized> Handle<T> {
    /// Get the index this handle references
    pub(in crate::util::arena) fn index(&self) -> u32 {
        self.index
    }

    /// Get the generation this handle references
    pub(in crate::util::arena) fn generation(&self) -> u32 {
        self.generation
    }
}
