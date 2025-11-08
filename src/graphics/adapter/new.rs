use crate::{
    Error, Result,
    graphics::{Adapter, Output},
};
use win32::{
    ComPtr,
    dxgi::{DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG, IDXGIAdapter1},
    try_hresult,
};

impl Adapter {
    /// Create a new [`Adapter`] if the adapter has outputs and is real hardware
    pub(in crate::graphics::adapter) fn new(
        mut adapter: ComPtr<IDXGIAdapter1>,
    ) -> Result<Option<Self>> {
        let mut desc = DXGI_ADAPTER_DESC1::default();
        try_hresult!(adapter.get_desc1(&mut desc))
            .map_err(|os| Error::new_inner("unable to get adapter description", os))?;

        // Check if the adapter is a software adapter
        let is_software = desc.flags & DXGI_ADAPTER_FLAG::Software as u32 != 0;
        if is_software {
            return Ok(None);
        }

        // Enumerate outputs
        let outputs = Output::enumerate(&mut adapter)?;

        // Extract the name
        let mut name_len = desc.description.len();
        for i in 0..desc.description.len() {
            if desc.description[i] == 0 {
                name_len = i;
                break;
            }
        }
        let name = String::from_utf16_lossy(&desc.description[..name_len]);

        Ok(Some(Adapter {
            name,
            video_memory: desc.dedicated_video_memory,
            outputs,
            adapter,
        }))
    }
}
