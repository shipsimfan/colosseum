use crate::update::ecs::archetype::Components;

impl Drop for Components {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            debug_assert!(self.capacity == 0);
            return;
        }

        for i in 0..self.count {
            let ptr = unsafe { self.ptr.add(i * self.layout.size()) };
            unsafe { (self.drop_fn)(ptr) };
        }

        unsafe { std::alloc::dealloc(self.ptr, self.layout) };
    }
}
