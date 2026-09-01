use crate::{Error, Result, render::PerFrameObjectBuilder};

impl<'a> PerFrameObjectBuilder<'a> {
    /// Add a new per-frame descriptor set
    pub fn add_descriptor_set(&mut self, descriptor_set_layout: usize, index: usize) -> Result<()> {
        assert_eq!(index, self.descriptor_sets.len());

        let descriptor_set_layout = self
            .fixed_render_objects
            .descriptor_set_layout(descriptor_set_layout);

        let descriptor_set = self
            .descriptor_pool
            .allocate_descriptor_set(descriptor_set_layout)
            .map_err(Error::new_inner)?;

        self.descriptor_sets.push(descriptor_set);
        Ok(())
    }
}
