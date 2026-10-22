use crate::{
    Result,
    render::{
        DescriptorSetLayouts, FullscreenQuadShader, GraphicsDevice, Pipeline, PostProcess, Shader,
        ShaderKind,
        frame_graph::{QuantizationNode, nodes::quantization::PushConstants},
    },
};
use alexandria::gpu::compile_shader;
use colosseum_ecs::World;

compile_shader! {
    const FRAGMENT_SHADER = "quantization.slang",
    frag_main
}

impl QuantizationNode {
    /// Create the resources required by the [`QuantizationNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = Shader::new(&FRAGMENT_SHADER, ShaderKind::Other, device)?;

        let fullscreen_quad = world.get_resource::<FullscreenQuadShader>();
        let descriptor_set_layouts = world.get_resource::<DescriptorSetLayouts>();
        let descriptor_set_layout = descriptor_set_layouts.get::<PostProcess>();

        let pipeline = Pipeline::<QuantizationNode>::new_post_process(
            "Quantization Pipeline",
            fullscreen_quad.get(),
            &shader,
            std::mem::size_of::<PushConstants>(),
            None,
            &[device.swapchain_format()],
            &[descriptor_set_layout],
            &device,
        )?;

        drop(fullscreen_quad);

        world.insert_resource(pipeline);

        Ok(())
    }
}
