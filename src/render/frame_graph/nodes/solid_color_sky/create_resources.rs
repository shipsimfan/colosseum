use crate::{
    Result,
    render::{
        FullscreenQuadShader, GraphicsDevice, HDR_FORMAT, Pipeline, Shader, ShaderKind,
        frame_graph::SolidColorSkyNode,
    },
};
use alexandria::{
    gpu::compile_shader,
    math::{Color4f, Linear},
};
use colosseum_ecs::World;

compile_shader! {
    const FRAGMENT_SHADER = "solid-color-sky.slang",
    frag_main
}

impl SolidColorSkyNode {
    /// Create the resources required by the [`SolidColorSkyNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = Shader::new(&FRAGMENT_SHADER, ShaderKind::Other, device)?;

        let fullscreen_quad = world.get_resource::<FullscreenQuadShader>();
        let pipeline = Pipeline::<SolidColorSkyNode>::new_post_process(
            "Solid Color Sky Pipeline",
            fullscreen_quad.get(),
            &shader,
            std::mem::size_of::<Color4f<Linear>>(),
            None,
            &[HDR_FORMAT],
            &[],
            &device,
        )?;

        drop(fullscreen_quad);

        world.insert_resource(pipeline);

        Ok(())
    }
}
