use crate::{TransformHandle, graphics::MeshRenderer, math::Matrix4x4f};

impl MeshRenderer {
    /// Get the number of instances in the mesh renderer
    pub const fn num_instances(&self) -> usize {
        self.instances.len()
    }

    /// Push a new instance into this renderer
    pub fn push(&mut self, transform: TransformHandle) -> usize {
        assert!(self.num_instances() < self.instance_buffer.len());
        let i = self.num_instances();
        self.instance_buffer[i] = Matrix4x4f::identity();
        self.instances.push((transform, 0));
        i
    }

    /// Insert new instance at index `i`
    pub fn insert(&mut self, i: usize, transform: TransformHandle) -> usize {
        assert!(self.num_instances() < self.instance_buffer.len());
        assert!(i <= self.num_instances());
        self.instances.insert(i, (transform, 0));
        for j in (i..self.num_instances()).rev() {
            let instance = self.instance_buffer[j];
            self.instance_buffer[j + 1] = instance;
        }
        self.instance_buffer[i] = Matrix4x4f::identity();
        i
    }

    /// Remove the last instance from this renderer
    pub fn pop(&mut self) -> Option<TransformHandle> {
        if self.num_instances() == 0 {
            return None;
        }

        self.instances.pop().map(|(handle, _)| handle)
    }

    /// Removes the instance at index `i` and replaces it with the one at the end of the list
    pub fn swap_remove(&mut self, i: usize) {
        assert!(i < self.num_instances());

        let instance = self.instance_buffer[self.num_instances()];
        self.instance_buffer[i] = instance;
        self.instances.swap_remove(i);
    }

    /// Remove the instance at `i`, moving all instances after it over by one
    pub fn remove(&mut self, i: usize) -> TransformHandle {
        assert!(i < self.num_instances());
        let handle = self.instances.remove(i).0;
        for j in i..self.num_instances() {
            let instance = self.instance_buffer[j + 1];
            self.instance_buffer[j] = instance;
        }
        handle
    }
}
