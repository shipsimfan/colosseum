use crate::{Error, Result, logging::Logger};
use alexandria::{
    cargo_vulkan_version,
    gpu::{GpuSubsystem, VulkanInstance, VulkanVersion},
    window::Window,
};

#[cfg(debug_assertions)]
mod debug;
#[cfg(not(debug_assertions))]
mod release;

#[cfg(debug_assertions)]
use debug::*;
#[cfg(not(debug_assertions))]
use release::*;

pub(in crate::run::wsi::new) fn create(
    gpu: &GpuSubsystem,
    logger: &Logger,
    game_name: &str,
    game_version: VulkanVersion,
    window: &Window<()>,
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
        .map_err(|error| Error::new_inner(error))?;

    Ok((vulkan_instance, create_debug_messenger))
}
