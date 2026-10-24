use crate::{
    Result,
    render::{
        DescriptorSetLayouts, FullscreenQuadShader, GraphicsDevice, Pipeline, PostProcess,
        SDR_FORMAT, Shader, ShaderKind, frame_graph::FxaaNode,
    },
};
use alexandria::{gpu::compile_shader, math::Vector2f};
use colosseum_ecs::World;

compile_shader! {
    const FRAGMENT_SHADER = "fxaa.slang",
    frag_main
}

impl FxaaNode {
    /// Create the resources required by the [`FxaaNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = Shader::new(&FRAGMENT_SHADER, ShaderKind::Other, device)?;

        let fullscreen_quad = world.get_resource::<FullscreenQuadShader>();
        let descriptor_set_layouts = world.get_resource::<DescriptorSetLayouts>();
        let descriptor_set_layout = descriptor_set_layouts.get::<PostProcess>();

        let pipeline = Pipeline::<FxaaNode>::new_post_process(
            "FXAA Pipeline",
            fullscreen_quad.get(),
            &shader,
            std::mem::size_of::<Vector2f>(),
            None,
            &[SDR_FORMAT],
            &[descriptor_set_layout],
            &device,
        )?;

        drop(fullscreen_quad);

        world.insert_permanent_resource(pipeline);

        Ok(())
    }
}
