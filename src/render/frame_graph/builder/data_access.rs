use crate::render::frame_graph::FrameGraphBuilder;
use std::any::TypeId;

impl<'a> FrameGraphBuilder<'a> {
    /// Indicate that a descriptor accesses a data buffer
    pub fn data_access<T: 'static>(&mut self, descriptor_set: usize, binding: u32) {
        let r#type = TypeId::of::<T>();
        for (data_buffer_type, bindings) in &mut self.data_bindings {
            if *data_buffer_type == r#type {
                bindings.push((descriptor_set, binding));
                return;
            }
        }

        panic!(
            "no data buffer found for \"{}\"",
            std::any::type_name::<T>()
        )
    }
}
