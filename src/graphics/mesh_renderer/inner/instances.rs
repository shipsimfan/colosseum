use crate::{
    graphics::MeshRendererInner,
    math::{Matrix4x4f, Transform},
};

impl MeshRendererInner {
    /// Get the number of instances in the mesh renderer
    pub const fn num_instances(&self) -> usize {
        self.instances.len()
    }

    /// Update instance `i` with `transform`
    pub fn update<T: AsMut<Transform>>(&mut self, i: usize, mut transform: T) {
        let transform = transform.as_mut();
        transform.update();

        self.dirty = true;
        self.instances[i] = transform.matrix();
    }

    /// Push a new instance into this renderer
    pub fn push(&mut self) -> usize {
        assert!(self.instances.len() < self.max_instances);
        self.instances.push(Matrix4x4f::identity());
        self.dirty = true;
        self.instances.len() - 1
    }

    /// Insert new instance at index `i`
    pub fn insert(&mut self, i: usize) -> usize {
        assert!(self.instances.len() < self.max_instances);
        self.instances.insert(i, Matrix4x4f::identity());
        self.dirty = true;
        i
    }

    /// Remove the last instance from this renderer
    pub fn pop(&mut self) -> bool {
        self.instances.pop().is_some()
    }

    /// Removes the instance at index `i` and replaces it with the one at the end of the list
    pub fn swap_remove(&mut self, i: usize) {
        self.dirty = true;
        self.instances.swap_remove(i);
    }

    /// Remove the instance at `i`, moving all instances after it over by one
    pub fn remove(&mut self, i: usize) {
        self.dirty = true;
        self.instances.remove(i);
    }
}
