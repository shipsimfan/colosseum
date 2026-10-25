//! Items used in rendering

use alexandria::gpu::VulkanFormat;
use fixed_data::*;
use frame_graph::*;
use objects::*;
use per_frame_data::*;

mod fixed_data;
mod frame_graph;
mod init;
mod objects;
mod per_frame_data;
mod phases;
mod public_data;
mod stop;
mod transfer;

pub use public_data::*;

pub use alexandria::gpu::{VulkanShaderModuleCode as ShaderCode, compile_shader};

pub(crate) use init::*;
pub(crate) use phases::execute_render_systems;
pub(crate) use stop::*;
pub(crate) use transfer::*;

/// The maximum number of frames that can be processed concurrently
const MAX_FRAMES_IN_FLIGHT: usize = 3;

/// The format to use for all depth attachments in the render graph
const DEPTH_FORMAT: VulkanFormat = VulkanFormat::D32Sfloat;

/// The format to use for all SDR attachments in the render graph
const SDR_FORMAT: VulkanFormat = VulkanFormat::R16G16B16A16Unorm;

/// The format to use for all HDR attachments in the render graph
const HDR_FORMAT: VulkanFormat = VulkanFormat::R16G16B16A16Sfloat;
