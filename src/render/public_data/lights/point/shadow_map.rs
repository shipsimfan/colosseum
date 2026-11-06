use crate::{
    Result,
    render::{
        FrameGraphResourceId, GpuPointLight, GraphicsDevice, PointLight, PointLightDsl, Shader,
        ShadowMapLight,
    },
};
use alexandria::{
    gpu::{VulkanPipelineShaderStageCreateInfo, VulkanShaderStageFlag, compile_shader},
    math::{Color3f, Srgb},
};

compile_shader! {
    const SHADER = "shadow-map.slang",
    vert_main,
    frag_main
}

impl ShadowMapLight for PointLight {
    const NAME: &'static str = "Point Light";
    const COLOR: Color3f<Srgb> = (0.0, 0.0, 0.0).into();

    const SHADOW_MAP: FrameGraphResourceId = FrameGraphResourceId::POINT_LIGHT_SHADOW_MAP;
    const CASCADES: u32 = 6;

    type Gpu = GpuPointLight;
    type DescriptorSetLayout = PointLightDsl;

    fn create_shader(device: &GraphicsDevice) -> Result<Shader> {
        Shader::new(&SHADER, device)
    }

    fn shader_stages<'a>(shader: &'a Shader) -> Vec<VulkanPipelineShaderStageCreateInfo<'a>> {
        vec![
            VulkanPipelineShaderStageCreateInfo::new(
                0,
                VulkanShaderStageFlag::Vertex,
                shader.module(),
                SHADER.entry_points()[0],
                None,
            ),
            VulkanPipelineShaderStageCreateInfo::new(
                0,
                VulkanShaderStageFlag::Fragment,
                shader.module(),
                SHADER.entry_points()[1],
                None,
            ),
        ]
    }
}
