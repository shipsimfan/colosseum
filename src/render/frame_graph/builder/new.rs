use crate::render::{
    DescriptorSetLayouts, GpuDataBuffers, GraphicsDevice,
    frame_graph::{FrameGraphBuilder, builder::FrameGraphBuilderResources},
    phases::FrameGraph,
};
use colosseum_ecs::Commands;

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new [`FrameGraphBuilder`]
    pub fn new(
        commands: &'a mut Commands,
        device: &'a GraphicsDevice,
        data_buffers: &'a GpuDataBuffers,
        descriptor_set_layouts: &DescriptorSetLayouts,
    ) -> FrameGraphBuilder<'a> {
        // Clear the frame graph phase
        commands.clear_phase(FrameGraph);

        // Initialize the frame graph builder with the provided resources
        let mut builder = FrameGraphBuilder {
            commands,
            device,
            data_buffers,

            nodes: Vec::new(),
            pipeline_barriers: Vec::new(),
            resources: FrameGraphBuilderResources::new(),
            descriptors: Vec::new(),
            descriptor_sets: Vec::new(),
            data_bindings: Vec::new(),

            current_pipeline_barriers: 0,

            color_attachment: None,
            depth_attachment: None,
        };

        // Setup the descriptor sets which reference data buffers
        for layout in descriptor_set_layouts {
            if layout.data_bindings().is_empty() {
                continue;
            }

            let descriptor_set = builder.create_descriptor_set_direct(
                format!("{} Descriptor Set", layout.name()),
                layout.r#type(),
                layout.bindings(),
            );

            for &(r#type, name, binding) in layout.data_bindings() {
                builder.data_access(r#type, name, descriptor_set, binding);
            }
        }

        builder
    }
}
