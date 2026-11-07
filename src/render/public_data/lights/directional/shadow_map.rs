use crate::{
    Result,
    render::{
        DirectionalLight, DirectionalLightDsl, FrameGraphResourceId, GpuDirectionalLight,
        GraphicsDevice, Shader, ShadowMapLight,
    },
};
use alexandria::{
    gpu::{VulkanPipelineShaderStageCreateInfo, VulkanShaderStageFlag, compile_shader},
    math::{Color3f, Srgb},
};

compile_shader! {
    const SHADER = "shadow-map.slang",
    vert_main
}

impl ShadowMapLight for DirectionalLight {
    const NAME: &'static str = "Directional Light";
    const COLOR: Color3f<Srgb> = (0.55, 0.30, 0.75).into();

    const SHADOW_MAP: FrameGraphResourceId = FrameGraphResourceId::DIRECTIONAL_LIGHT_SHADOW_MAP;
    const CASCADES: u32 = 4;

    type Gpu = GpuDirectionalLight;
    type DescriptorSetLayout = DirectionalLightDsl;

    fn create_shader(device: &GraphicsDevice) -> Result<Shader> {
        Shader::new(&SHADER, device)
    }

    fn shader_stages<'a>(shader: &'a Shader) -> Vec<VulkanPipelineShaderStageCreateInfo<'a>> {
        vec![VulkanPipelineShaderStageCreateInfo::new(
            0,
            VulkanShaderStageFlag::Vertex,
            shader.module(),
            SHADER.entry_points()[0],
            None,
        )]
    }
}
