use crate::{
    Error, Result,
    graphics::context::{LightType, managed_objects::lights::LightList},
};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_BUFFEREX_SRV, D3D11_CPU_ACCESS_FLAG, D3D11_MAP,
        D3D11_MAPPED_SUBRESOURCE, D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SHADER_RESOURCE_VIEW_DESC_UNION, D3D11_SRV_DIMENSION, D3D11_SUBRESOURCE_DATA,
        D3D11_USAGE, ID3D11Device, ID3D11DeviceContext,
    },
    dxgi::DXGI_FORMAT,
    try_hresult,
};

impl<T: LightType> LightList<T> {
    /// Bind this light list to the render pipeline, returning the new number of active lights if
    /// the amount changed
    pub fn bind(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<Option<usize>> {
        let mut lights = self.shared_list.borrow_mut();

        // Check if the list changed
        let changed_length = self.buffer_cache.len() != lights.0.len();
        let resize = self.buffer_capacity < lights.0.len();
        let mut remap = if lights.1 {
            for (i, light) in lights.0.iter_mut().enumerate() {
                let mut light = light.borrow_mut();
                light.update();
                if i >= self.buffer_cache.len() {
                    self.buffer_cache.push(light.to_gpu());
                } else {
                    self.buffer_cache[i] = light.to_gpu();
                }
            }

            self.buffer_cache.truncate(lights.0.len());

            true
        } else {
            // Check if any light changed
            let mut dirty = false;
            for (i, light) in lights.0.iter_mut().enumerate() {
                let mut light = light.borrow_mut();
                let light_dirty = light.update();
                dirty |= light_dirty;
                if light_dirty {
                    self.buffer_cache[i] = light.to_gpu();
                }
            }

            dirty
        };
        if resize {
            assert!(remap);
            remap = false;
        }

        // Resize the buffer if needed
        if resize {
            while lights.0.len() > self.buffer_capacity {
                self.buffer_capacity *= 2;
            }

            let buffer_desc = D3D11_BUFFER_DESC {
                byte_width: (std::mem::size_of::<T::GPU>() * self.buffer_capacity) as _,
                usage: D3D11_USAGE::Dynamic,
                bind_flags: D3D11_BIND_FLAG::ShaderResource as _,
                cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
                misc_flags: 0,
                structure_byte_stride: std::mem::size_of::<T::GPU>() as _,
            };

            let initial_data = D3D11_SUBRESOURCE_DATA {
                sys_mem: self.buffer_cache.as_ptr().cast(),
                sys_mem_pitch: 0,
                sys_mem_slice_pitch: 0,
            };

            self.buffer = ComPtr::new_in(|buffer| {
                try_hresult!(device.create_buffer(&buffer_desc, &initial_data, buffer))
            })
            .map_err(|error| Error::new_inner("unable to create light list buffer", error))?;

            let shader_resource_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                format: DXGI_FORMAT::Unknown,
                view_dimension: D3D11_SRV_DIMENSION::BufferEx,
                u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
                    buffer_ex: D3D11_BUFFEREX_SRV {
                        first_element: 0,
                        num_elements: self.buffer_capacity as _,
                        flags: 0,
                    },
                },
            };

            self.buffer_view = ComPtr::new_in(|shader_resource_view| {
                try_hresult!(device.create_shader_resource_view(
                    self.buffer.as_mut(),
                    &shader_resource_desc,
                    shader_resource_view
                ))
            })
            .map_err(|error| Error::new_inner("unable to create light list buffer view", error))?;
        }

        // Remap the buffer if needed
        if remap {
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map camera constant buffer", error))?;

            let dest = unsafe {
                std::slice::from_raw_parts_mut(
                    mapped_resource.data as *mut T::GPU,
                    self.buffer_cache.len(),
                )
            };
            dest.copy_from_slice(&self.buffer_cache);

            device_context.unmap(self.buffer.as_mut(), 0);
        }

        // Bind the buffer
        let view = self.buffer_view.as_mut() as *mut _;
        device_context.vs_set_shader_resources(self.buffer_slot, 1, &view);
        device_context.ps_set_shader_resources(self.buffer_slot, 1, &view);

        Ok(if changed_length {
            Some(self.buffer_cache.len())
        } else {
            None
        })
    }
}
