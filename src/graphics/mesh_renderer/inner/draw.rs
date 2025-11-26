use crate::{Error, Result, graphics::MeshRendererInner, math::Matrix4x4f};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11Buffer, ID3D11Device, ID3D11DeviceContext},
    try_hresult,
};

impl MeshRendererInner {
    /// Draw this mesh using the active settings
    pub(in crate::graphics) fn draw(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        // Skip rendering if theres nothing to render
        if self.instances.len() == 0 {
            return Ok(());
        }

        // Update the instance buffer if needed
        if self.dirty {
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.instance_buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map instance buffer", error))?;

            let dest = unsafe {
                std::slice::from_raw_parts_mut(
                    mapped_resource.data as *mut Matrix4x4f,
                    self.instances.len(),
                )
            };
            dest.copy_from_slice(&self.instances);
            device_context.unmap(self.instance_buffer.as_mut(), 0);
            self.dirty = false;
        }

        // Bind the mesh
        self.mesh.bind(device, device_context)?;

        // Bind the instance buffer
        let buffer = self.instance_buffer.as_mut() as *mut ID3D11Buffer;
        let stride = std::mem::size_of::<Matrix4x4f>() as _;
        let offset = 0;
        device_context.ia_set_vertex_buffers(1, 1, &buffer, &stride, &offset);

        // Draw
        device_context.draw_indexed_instanced(
            self.mesh.indices().len() as _,
            self.instances.len() as _,
            0,
            0,
            0,
        );
        Ok(())
    }
}
