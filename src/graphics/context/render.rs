use crate::{Error, Result, graphics::GraphicsContext, math::Color3f};
use win32::{UINT, d3d11::D3D11_PRIMITIVE_TOPOLOGY, dxgi::DXGI_PRESENT_ALLOW_TEARING, try_hresult};

impl GraphicsContext {
    /// Render the current state as a frame
    pub(crate) fn render(&mut self, clear_color: Color3f) -> Result<()> {
        // Resize if needed
        self.resize()?;

        // Clear debug message queue from object creation
        self.log_debug_messages()?;

        // Clear render target view
        self.swapchain_objects.as_mut().unwrap().clear(
            &mut self.device_context,
            [clear_color.r, clear_color.g, clear_color.b, 0.0],
        );

        // Set global render state variables
        self.device_context
            .ia_set_primitive_topology(D3D11_PRIMITIVE_TOPOLOGY::TriangleList);
        self.device_context
            .rs_set_state(self.rasterizer_state.as_mut());
        self.device_context
            .om_set_blend_state(self.blend_state.as_mut(), [1.; 4], UINT::MAX);
        self.device_context
            .om_set_depth_stencil_state(self.depth_stencil_state.as_mut(), 0);

        // Bind swapchain
        self.swapchain_objects
            .as_mut()
            .unwrap()
            .bind(&mut self.device_context);

        // TODO: Lighting pre-passes

        // Camera render passes
        let mut active_shader = 0;
        for camera in &*self.managed_objects.cameras() {
            let mut camera = camera.borrow_mut();
            if !camera.is_active() {
                continue;
            }

            camera.bind(self.size, &mut self.device_context)?;

            // Opaque render pass
            for material in &*self.managed_objects.opaque_materials() {
                let mut material = material.borrow_mut();
                let shader = material.shader();
                if shader.id().get() != active_shader {
                    shader.bind(&mut self.device_context);
                    active_shader = shader.id().get();
                }

                material.render(&mut self.device_context)?;
            }

            // TODO: Transparent render pass
        }

        // TODO: Post-process, anti-aliasing, and render scaling

        // TODO: UI render pass

        // Present
        try_hresult!(self.swapchain.present(
            if self.vsync { 1 } else { 0 },
            if self.vsync {
                0
            } else {
                DXGI_PRESENT_ALLOW_TEARING
            }
        ))
        .map_err(|error| Error::new_inner("unable to render frame", error))?;

        // Log render messages
        self.log_debug_messages()?;

        Ok(())
    }
}
