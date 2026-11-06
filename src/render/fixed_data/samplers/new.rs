use std::sync::Arc;

use alexandria::gpu::{
    VulkanBorderColor, VulkanCompareOp, VulkanFilter, VulkanSamplerAddressMode,
    VulkanSamplerMipmapMode,
};
use colosseum_core::new_error;

use crate::{
    Result,
    render::{GraphicsDevice, Samplers},
};

impl Samplers {
    /// Create a new set of [`Samplers`]
    pub fn new(device: &GraphicsDevice) -> Result<Samplers> {
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut linear = device
            .create_sampler(
                0,
                VulkanFilter::Linear,
                VulkanFilter::Linear,
                VulkanSamplerMipmapMode::Linear,
                VulkanSamplerAddressMode::ClampToEdge,
                VulkanSamplerAddressMode::ClampToEdge,
                VulkanSamplerAddressMode::ClampToEdge,
                0.0,
                false,
                1.0,
                false,
                VulkanCompareOp::Never,
                0.0,
                0.0,
                VulkanBorderColor::FloatTransparentBlack,
                false,
            )
            .map_err(|error| new_error!("unable to create the linear sampler - {}", error))?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut linear, c"Linear Sampler")
            .map_err(|error| new_error!("unable to name the linear sampler - {}", error))?;

        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut pcf = device
            .create_sampler(
                0,
                VulkanFilter::Linear,
                VulkanFilter::Linear,
                VulkanSamplerMipmapMode::Nearest,
                VulkanSamplerAddressMode::ClampToBorder,
                VulkanSamplerAddressMode::ClampToBorder,
                VulkanSamplerAddressMode::ClampToBorder,
                0.0,
                false,
                1.0,
                true,
                VulkanCompareOp::GreaterOrEqual,
                0.0,
                1.0,
                VulkanBorderColor::FloatOpaqueWhite,
                false,
            )
            .map_err(|error| new_error!("unable to create the pcf sampler - {}", error))?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut pcf, c"PCF Sampler")
            .map_err(|error| new_error!("unable to name the pcf sampler - {}", error))?;

        Ok(Samplers {
            linear: Arc::new(linear),
            pcf: Arc::new(pcf),
        })
    }
}
