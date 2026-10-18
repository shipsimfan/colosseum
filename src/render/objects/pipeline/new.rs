use crate::{
    Result,
    render::{GraphicsDevice, Pipeline, Shader},
};
use alexandria::gpu::{
    VulkanDescriptorSetLayout, VulkanFormat, VulkanPipelineColorBlendStateCreateInfo,
    VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
    VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineMultisampleStateCreateInfo,
    VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineRenderingCreateInfo,
    VulkanPipelineShaderStageCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
    VulkanPipelineViewportStateCreateInfo, VulkanPushConstantRange,
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;
use std::{marker::PhantomData, sync::Arc};

impl<Kind> Pipeline<Kind> {
    /// Create a completely custom pair of pipeline and layout
    pub fn new(
        name: &'static str,
        descriptor_set_layouts: &[&VulkanDescriptorSetLayout],
        push_constant_ranges: &[VulkanPushConstantRange],
        output_formats: &[VulkanFormat],
        depth_format: VulkanFormat,
        shader_stages: &[VulkanPipelineShaderStageCreateInfo],
        shaders: Vec<Arc<Shader>>,
        vertex_input_state: &VulkanPipelineVertexInputStateCreateInfo,
        input_assembly_state: &VulkanPipelineInputAssemblyStateCreateInfo,
        viewport_state: &VulkanPipelineViewportStateCreateInfo,
        rasterization_state: &VulkanPipelineRasterizationStateCreateInfo,
        multisample_state: &VulkanPipelineMultisampleStateCreateInfo,
        depth_stencil_state: Option<&VulkanPipelineDepthStencilStateCreateInfo>,
        color_blend_state: &VulkanPipelineColorBlendStateCreateInfo,
        dynamic_state: &VulkanPipelineDynamicStateCreateInfo,
        device: &GraphicsDevice,
    ) -> Result<Pipeline<Kind>> {
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut pipeline_layout = device
            .create_pipeline_layout(0, descriptor_set_layouts, push_constant_ranges)
            .map_err(|error| {
                new_error!(
                    "unable to create pipeline layout for \"{}\" pipeline - {}",
                    name,
                    error
                )
            })?;
        #[cfg(debug_assertions)]
        let layout_name = CString::new(format!("{} Layout", name)).unwrap();
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut pipeline_layout, &layout_name)
            .map_err(|error| {
                new_error!(
                    "unable to set object name for pipeline layout for \"{}\" pipeline - {}",
                    name,
                    error
                )
            })?;

        // Actually create the graphics pipeline
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut pipeline = device
            .create_graphics_pipeline(
                [&mut VulkanPipelineRenderingCreateInfo::new(
                    0,
                    output_formats,
                    depth_format,
                    VulkanFormat::Undefined,
                ) as _],
                None,
                0,
                &shader_stages,
                Some(&vertex_input_state),
                Some(&input_assembly_state),
                None,
                Some(&viewport_state),
                Some(&rasterization_state),
                Some(&multisample_state),
                depth_stencil_state,
                Some(&color_blend_state),
                Some(&dynamic_state),
                Some(&pipeline_layout),
                None,
                0,
                None,
                0,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create graphics \"{}\" pipeline - {}",
                    name,
                    error
                )
            })?;
        #[cfg(debug_assertions)]
        let pipeline_name = CString::new(name).unwrap();
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut pipeline, &pipeline_name)
            .map_err(|error| {
                new_error!(
                    "unable to set object name for \"{}\" pipeline - {}",
                    name,
                    error
                )
            })?;

        Ok(Pipeline {
            pipeline,
            layout: pipeline_layout,
            shaders,
            _kind: PhantomData,
        })
    }
}
