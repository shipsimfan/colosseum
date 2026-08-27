//! Items used in rendering

use alexandria::gpu::VulkanFormat;
use as_bytes::*;
use frame_graph::*;
use pipeline::*;

mod as_bytes;
mod data;
mod frame_graph;
mod job;
mod material;
mod mesh;
mod pipeline;
mod render_objects;
mod transfer;

pub use data::*;
pub use material::*;
pub use mesh::*;
pub use transfer::*;

pub(crate) use job::*;
pub(crate) use render_objects::*;

pub use alexandria::gpu::{VulkanShaderModuleCode as ShaderCode, compile_shader};

/// The format to use for all depth attachments in the render graph
const DEPTH_FORMAT: VulkanFormat = VulkanFormat::D32SFloat;

/// The format to use for all SDR attachments in the render graph
const SDR_FORMAT: VulkanFormat = VulkanFormat::R16G16B16A16UNorm;

/// The format to use for all HDR attachments in the render graph
const HDR_FORMAT: VulkanFormat = VulkanFormat::R16G16B16A16SFloat;
