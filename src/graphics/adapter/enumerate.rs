use crate::{Error, Result, graphics::Adapter};
use std::{cmp::Ordering, ptr::null_mut};
use win32::{
    ComInterface, ComPtr, DXGI_ERROR_NOT_FOUND, UINT, dxgi1_2::IDXGIFactory2,
    dxgi1_3::CreateDXGIFactory2, strsafe::S_OK, try_hresult,
};

#[cfg(debug_assertions)]
const FACTORY_FLAGS: UINT = win32::dxgi1_3::DXGI_CREATE_FACTORY_DEBUG;

#[cfg(not(debug_assertions))]
const FACTORY_FLAGS: UINT = 0;

impl Adapter {
    /// Enumerate the adapters on the system
    pub fn enumerate() -> Result<Vec<Self>> {
        // Create the factory
        let mut factory = ComPtr::<IDXGIFactory2>::new_in(|factory| {
            try_hresult!(CreateDXGIFactory2(
                FACTORY_FLAGS,
                &IDXGIFactory2::IID,
                factory.cast()
            ))
        })
        .map_err(|os| Error::new_inner("unable to start DXGI", os))?;

        // Enumerate the adapters
        let mut adapters = Vec::new();
        let mut idx = 0;
        loop {
            let mut adapter = null_mut();
            let result = factory.enum_adapters1(idx, &mut adapter);
            idx += 1;
            if result == S_OK {
                if let Some(adapter) = Adapter::new(ComPtr::new(adapter))? {
                    adapters.push(adapter);
                }
            } else if result == DXGI_ERROR_NOT_FOUND {
                break;
            } else {
                return Err(Error::new_inner(
                    "unable to enumerate adapters",
                    win32::Error::new(result),
                ));
            }
        }

        // Sort adapters into recommended order:
        //  1. More Video Memory
        //  2. Alphabetical
        adapters.sort_by(|a, b| match b.video_memory.cmp(&a.video_memory) {
            Ordering::Equal => a.name.cmp(&b.name),
            ordering => ordering,
        });

        Ok(adapters)
    }
}
