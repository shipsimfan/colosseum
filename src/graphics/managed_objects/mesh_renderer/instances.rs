use crate::{
    graphics::MeshRenderer,
    math::{Matrix4x4f, Transform},
};

impl MeshRenderer {
    /// Get the number of instances in the mesh renderer
    pub const fn num_instances(&self) -> usize {
        self.active_instances
    }

    /// Update instance `i` with `transform`
    pub fn update<T: AsMut<Transform>>(&mut self, i: usize, mut transform: T) {
        assert!(i < self.active_instances);

        let transform = transform.as_mut();
        transform.update();

        self.instance_buffer[i] = transform.matrix();
    }

    /// Push a new instance into this renderer
    pub fn push(&mut self) -> usize {
        assert!(self.active_instances < self.instance_buffer.len());
        self.instance_buffer[self.active_instances] = Matrix4x4f::identity();
        self.active_instances += 1;
        self.active_instances - 1
    }

    /// Insert new instance at index `i`
    pub fn insert(&mut self, i: usize) -> usize {
        assert!(self.active_instances < self.instance_buffer.len());
        assert!(i <= self.active_instances);
        for j in (i..self.active_instances).rev() {
            let instance = self.instance_buffer[j];
            self.instance_buffer[j + 1] = instance;
        }
        self.instance_buffer[i] = Matrix4x4f::identity();
        i
    }

    /// Remove the last instance from this renderer
    pub fn pop(&mut self) -> bool {
        if self.active_instances == 0 {
            return false;
        }

        self.active_instances -= 1;
        true
    }

    /// Removes the instance at index `i` and replaces it with the one at the end of the list
    pub fn swap_remove(&mut self, i: usize) {
        assert!(i < self.active_instances);

        self.active_instances -= 1;
        let instance = self.instance_buffer[self.active_instances];
        self.instance_buffer[i] = instance;
    }

    /// Remove the instance at `i`, moving all instances after it over by one
    pub fn remove(&mut self, i: usize) {
        assert!(i < self.active_instances);
        self.active_instances -= 1;
        for j in i..self.active_instances {
            let instance = self.instance_buffer[j + 1];
            self.instance_buffer[j] = instance;
        }
    }
}
