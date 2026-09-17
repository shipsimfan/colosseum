use crate::{
    Result,
    render::{
        DeviceDataBuffer, FixedRenderObjects, PerFrameObjectBuilder, RenderData,
        RenderDirectionalLight, RenderObjects, RenderPointLight, RenderSpotLight, ShadowMapBuffer,
        frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, FrameGraphResources},
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDescriptorSet, VulkanDevice,
    VulkanFormat,
};
use r#macro::nodes;

mod r#macro;

mod fxaa;
mod lit_forward_render;
mod procedural_sky;
mod quantization;
mod render_scale;
mod shadow_map;
mod solid_color_sky;
mod tone_map;
mod unlit_forward_render;

pub(in crate::render::frame_graph) use fxaa::*;
pub(in crate::render::frame_graph) use lit_forward_render::*;
pub(in crate::render::frame_graph) use procedural_sky::*;
pub(in crate::render::frame_graph) use quantization::*;
pub(in crate::render::frame_graph) use render_scale::*;
pub(in crate::render::frame_graph) use shadow_map::*;
pub(in crate::render::frame_graph) use solid_color_sky::*;
pub(in crate::render::frame_graph) use tone_map::*;
pub(in crate::render::frame_graph) use unlit_forward_render::*;

nodes![
    simple: [
        /// A node that renders the sky as a solid color
        SolidColorSky(SolidColorSkyNode),

        /// A node that generates a procedural sky
        ProceduralSky(ProceduralSkyNode),

        /// A node that renders shadow maps for directional lights
        DirectionalLightShadowMap(ShadowMapNode<RenderDirectionalLight>),

        /// A node that renders shadow maps for point lights
        PointLightShadowMap(ShadowMapNode<RenderPointLight>),

        /// A node that renders shadow maps for spot lights
        SpotLightShadowMap(ShadowMapNode<RenderSpotLight>),
    ],

    data_buffer:[
        /// A node that renders unlit objects using a forward rendering pipeline
        UnlitForwardRender(UnlitForwardRenderNode),

        /// A node that renders lit objects using a forward rendering pipeline
        LitForwardRender(LitForwardRenderNode),
    ],

    post_process: [
        /// A node that changes the render scale of the input image and outputs it to a new image
        RenderScale(RenderScaleNode),

        /// A node that performs color correction, tone mapping, gamma correction, and color
        /// grading
        ToneMap(ToneMapNode),

        /// A node that performs sharpening and dithering before quantizing an input image
        Quantization(QuantizationNode),

        /// A node that performs FXAA anti-aliasing on an input image
        Fxaa(FxaaNode),
    ]
];
