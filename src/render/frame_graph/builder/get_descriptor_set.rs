use crate::render::frame_graph::FrameGraphBuilder;
use std::any::TypeId;

impl<'a> FrameGraphBuilder<'a> {
    /// Get a descriptor set for the specified type
    pub fn get_descriptor_set<T: 'static>(&mut self) -> usize {
        for (index, (descriptor_set, _, _)) in self.descriptor_sets.iter().enumerate() {
            if *descriptor_set == TypeId::of::<T>() {
                return index;
            }
        }

        panic!(
            "descriptor set not found for \"{}\"",
            std::any::type_name::<T>()
        );
    }
}
