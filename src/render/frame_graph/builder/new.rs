use crate::render::{
    GpuDataBuffers, GraphicsDevice,
    frame_graph::{FrameGraphBuilder, builder::FrameGraphBuilderResources},
    phases::FrameGraph,
};
use colosseum_ecs::Commands;

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new [`FrameGraphBuilder`]
    pub fn new(
        commands: &'a mut Commands,
        device: &'a GraphicsDevice,
        data_buffers: &GpuDataBuffers,
    ) -> FrameGraphBuilder<'a> {
        commands.clear_phase(FrameGraph);

        let mut data_bindings = Vec::with_capacity(data_buffers.len());
        for r#type in data_buffers.types() {
            data_bindings.push((r#type, Vec::new()));
        }

        FrameGraphBuilder {
            commands,
            device,

            nodes: Vec::new(),
            pipeline_barriers: Vec::new(),
            resources: FrameGraphBuilderResources::new(),
            descriptors: Vec::new(),
            descriptor_sets: Vec::new(),
            data_bindings,

            current_pipeline_barriers: 0,

            color_attachment: None,
            depth_attachment: None,
        }
    }
}
