use crate::graphics::{MeshRendererInner, Transform};

impl MeshRendererInner {
    /// Get the list of current instances
    pub const fn instances(&self) -> &[Transform] {
        self.instances.as_slice()
    }

    /// Get the instance at index `i`
    pub fn instance(&self, i: usize) -> Option<&Transform> {
        self.instances.get(i)
    }

    /// Get the number of instances in the mesh renderer
    pub const fn num_instances(&self) -> usize {
        self.instances.len()
    }

    /// Get the list of current instances mutably
    pub fn instances_mut(&mut self) -> &mut [Transform] {
        &mut self.instances
    }

    /// Get the instance at index `i` mutably
    pub fn instance_mut(&mut self, i: usize) -> Option<&mut Transform> {
        self.instances.get_mut(i)
    }

    /// Push a new instance into this renderer
    pub fn push(&mut self) -> &mut Transform {
        assert!(self.instances.len() < self.max_instances);
        self.instances.push(Transform::new());
        self.dirty = true;
        self.instances.last_mut().unwrap()
    }

    /// Insert new instance at index `i`
    pub fn insert(&mut self, i: usize) -> &mut Transform {
        assert!(self.instances.len() < self.max_instances);
        self.instances.insert(i, Transform::new());
        self.dirty = true;
        self.instances.last_mut().unwrap()
    }

    /// Remove the last instance from this renderer
    pub fn pop(&mut self) -> Option<Transform> {
        self.instances.pop()
    }

    /// Removes the instance at index `i` and replaces it with the one at the end of the list
    pub fn swap_remove(&mut self, i: usize) -> Transform {
        self.dirty = true;
        self.instances.swap_remove(i)
    }

    /// Remove the instance at `i`, moving all instances after it over by one
    pub fn remove(&mut self, i: usize) -> Transform {
        self.dirty = true;
        self.instances.remove(i)
    }
}
