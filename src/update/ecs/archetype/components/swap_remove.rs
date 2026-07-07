use crate::update::ecs::archetype::Components;

impl Components {
    /// Remove a component from the component data using the swap-remove method
    pub fn swap_remove(&mut self, index: usize) {
        debug_assert!(index < self.length, "component index out of bounds");

        self.length -= 1;

        let target_ptr = unsafe { self.ptr.add(index * self.element_size) };
        unsafe { (self.drop_fn)(target_ptr) };

        if index == self.length {
            return;
        }

        unsafe {
            let last_element_ptr = self.ptr.add(self.length * self.element_size);

            std::ptr::copy_nonoverlapping(last_element_ptr, target_ptr, self.layout.size());
        }
    }
}
