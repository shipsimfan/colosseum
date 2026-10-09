use crate::{Logger, Result, UserEvent};
use alexandria::{
    cargo_vulkan_version,
    gpu::{GpuSubsystem, VulkanInstance, VulkanVersion},
    window::Window,
};

#[cfg(debug_assertions)]
mod debug;
#[cfg(not(debug_assertions))]
mod release;

use colosseum_core::new_error;
#[cfg(debug_assertions)]
use debug::*;
#[cfg(not(debug_assertions))]
use release::*;

pub(in crate::run::wsi::new) fn create(
    gpu: &GpuSubsystem,
    logger: &Logger,
    game_name: &str,
    game_version: VulkanVersion,
    window: &Window<UserEvent>,
) -> Result<(VulkanInstance, bool)> {
    let (layers, extensions, create_debug_messenger) = get_layers_and_extensions(gpu, logger)?;

    let vulkan_instance = gpu
        .instance_builder(VulkanVersion::VERSION_1_3)
        .application(game_name, game_version)
        .engine("Colosseum", cargo_vulkan_version!())
        .layers(layers)
        .extensions(extensions)
        .window_extensions(window)
        .create()
        .map_err(|error| new_error!("unable to create Vulkan instance - {}", error))?;

    Ok((vulkan_instance, create_debug_messenger))
}
