use crate::util::arena::Slot;

impl<T: Sized> Slot<T> {
    /// Get the contained element
    pub fn get(&self, generation: u32) -> Option<&T> {
        if self.generation != generation {
            return None;
        }

        self.item.as_ref()
    }

    /// Get the contained element mutably
    pub fn get_mut(&mut self, generation: u32) -> Option<&mut T> {
        if self.generation != generation {
            return None;
        }

        self.item.as_mut()
    }

    /// Get the contained element without checking the generation
    pub fn get_unchecked(&self) -> Option<&T> {
        self.item.as_ref()
    }

    /// Get the contained element mutably without checking the generation
    pub fn get_unchecked_mut(&mut self) -> Option<&mut T> {
        self.item.as_mut()
    }

    /// Get the next free slot
    pub fn next_free(&self) -> u32 {
        self.next_free
    }

    /// Get the current generation
    pub fn generation(&self) -> u32 {
        self.generation
    }
}
