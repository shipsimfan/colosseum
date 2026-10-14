//! Items used in rendering

use frame_data::*;
use frame_index::*;
use graphics_device::*;
use swapchain::*;

mod frame_data;
mod frame_index;
mod graphics_device;
mod init;
mod phases;
mod swapchain;
mod transfer;

pub(crate) use init::*;
pub(crate) use phases::*;
pub(crate) use transfer::*;

const MAX_FRAMES_IN_FLIGHT: usize = 3;
