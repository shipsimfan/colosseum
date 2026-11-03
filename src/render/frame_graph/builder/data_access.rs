use crate::render::frame_graph::FrameGraphBuilder;
use std::any::TypeId;

impl<'a> FrameGraphBuilder<'a> {
    /// Indicate that a descriptor accesses a data buffer
    pub(in crate::render::frame_graph::builder) fn data_access(
        &mut self,
        r#type: TypeId,
        name: &'static str,
        descriptor_set: usize,
        binding: u32,
    ) {
        for (data_buffer_type, _, bindings) in &mut self.data_bindings {
            if *data_buffer_type == r#type {
                bindings.push((descriptor_set, binding));
                return;
            }
        }

        for (index, data_buffer) in self.data_buffers.types().enumerate() {
            if data_buffer == r#type {
                self.data_bindings
                    .push((r#type, index, vec![(descriptor_set, binding)]));
                return;
            }
        }

        panic!("unable to find data buffer of type \"{}\"", name);
    }
}
