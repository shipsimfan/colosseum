use crate::{
    Result,
    render::{
        DescriptorSetLayouts, FullscreenQuadShader, GraphicsDevice, Pipeline, PostProcess,
        SDR_FORMAT, Shader, ShaderKind,
        frame_graph::{ToneMapNode, nodes::tone_map::PushConstants},
    },
};
use alexandria::gpu::compile_shader;
use colosseum_ecs::World;

compile_shader! {
    const FRAGMENT_SHADER = "tone-map.slang",
    frag_main
}

impl<Game: crate::Game> ToneMapNode<Game> {
    /// Create the resources required by the [`ToneMapNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = Shader::new(&FRAGMENT_SHADER, ShaderKind::Other, device)?;

        let fullscreen_quad = world.get_resource::<FullscreenQuadShader>();
        let descriptor_set_layouts = world.get_resource::<DescriptorSetLayouts>();
        let descriptor_set_layout = descriptor_set_layouts.get::<PostProcess>();

        let pipeline = Pipeline::<ToneMapNode<Game>>::new_post_process(
            "Tone Map Pipeline",
            fullscreen_quad.get(),
            &shader,
            std::mem::size_of::<PushConstants>(),
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
