use crate::{
    Result,
    render::{
        FrameGraphResourceId, GpuSpotLight, GraphicsDevice, Shader, ShadowMapLight, SpotLight,
        SpotLightDsl,
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

impl ShadowMapLight for SpotLight {
    const NAME: &'static str = "Spot Light";
    const COLOR: Color3f<Srgb> = (0.0, 0.0, 0.0).into();

    const SHADOW_MAP: FrameGraphResourceId = FrameGraphResourceId::SPOT_LIGHT_SHADOW_MAP;
    const CASCADES: u32 = 1;

    type Gpu = GpuSpotLight;
    type DescriptorSetLayout = SpotLightDsl;

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
