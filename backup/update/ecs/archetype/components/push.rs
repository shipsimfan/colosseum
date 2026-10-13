use crate::update::ecs::archetype::Components;

impl Components {
    /// Push a new component into the component data
    pub fn push(&mut self, data: &[u8]) {
        debug_assert_eq!(
            data.len(),
            self.layout.size(),
            "component data size does not match component layout size"
        );

        if self.length == self.capacity {
            self.capacity = self.capacity * 2;

            self.ptr = unsafe {
                std::alloc::realloc(self.ptr, self.layout, self.capacity * self.element_size)
            };
        }

        let ptr = unsafe { self.ptr.add(self.length * self.element_size) };
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len()) };
        self.length += 1;
    }
}
