use crate::{Error, Result, graphics::Output};
use std::ptr::null_mut;
use win32::{ComPtr, DXGI_ERROR_NOT_FOUND, S_OK, dxgi::IDXGIAdapter1};

impl Output {
    /// Enumerates the outputs available for `adapter` to output on
    pub(in crate::graphics::adapter) fn enumerate(
        adapter: &mut ComPtr<IDXGIAdapter1>,
    ) -> Result<Vec<Self>> {
        let mut outputs = Vec::new();
        loop {
            let mut output = null_mut();
            let result = adapter.enum_outputs(outputs.len() as _, &mut output);
            if result == S_OK {
                outputs.push(Output::new(ComPtr::new(output), adapter.clone())?);
            } else if result == DXGI_ERROR_NOT_FOUND {
                break;
            } else {
                return Err(Error::new_inner(
                    "unable to enumerate adapter outputs",
                    win32::Error::new(result),
                ));
            }
        }

        Ok(outputs)
    }
}
