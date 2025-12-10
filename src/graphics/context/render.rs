use crate::{Error, ManagedObjects, Result, graphics::GraphicsContext, math::Color3f};
use win32::{UINT, d3d11::D3D11_PRIMITIVE_TOPOLOGY, dxgi::DXGI_PRESENT_ALLOW_TEARING, try_hresult};

impl GraphicsContext {
    /// Render the current state as a frame
    pub(crate) fn render(
        &mut self,
        managed_objects: &mut ManagedObjects,
        clear_color: Color3f,
    ) -> Result<()> {
        // Resize if needed
        self.resize(managed_objects)?;

        // Clear debug message queue from object creation
        self.log_debug_messages()?;

        // TODO: Lighting pre-passes

        // Bind global lighting information
        managed_objects
            .graphics
            .lights
            .bind(&self.device, &mut self.device_context)?;

        // Camera render passes
        let mut active_material = 0;
        for camera in &mut managed_objects.graphics.cameras {
            if !camera.is_active() {
                continue;
            }

            // Set pipeline state variables
            self.device_context
                .ia_set_primitive_topology(D3D11_PRIMITIVE_TOPOLOGY::TriangleList);
            self.device_context
                .rs_set_state(self.rasterizer_state.as_mut());
            self.device_context
                .om_set_blend_state(self.blend_state.as_mut(), [1.; 4], UINT::MAX);
            self.device_context
                .om_set_depth_stencil_state(self.depth_stencil_state.as_mut(), 0);

            // Bind the camera
            camera.bind(
                self.size,
                self.render_scale,
                self.anti_aliasing,
                [clear_color.r, clear_color.g, clear_color.b, 1.0],
                &self.device,
                &mut self.device_context,
            )?;

            // Opaque render pass
            for mesh_renderer in &mut managed_objects.graphics.mesh_renderers {
                let material =
                    &mut managed_objects.graphics.opaque_materials[mesh_renderer.material()];
                if material.id() != active_material {
                    material.bind(&mut self.device_context)?;
                    active_material = material.id();
                }

                mesh_renderer.draw(
                    &mut managed_objects.transforms,
                    &self.device,
                    &mut self.device_context,
                )?;
            }

            // TODO: Transparent render pass

            // Post-process, anti-aliasing, and render scaling
            self.post_processing.run(
                camera.post_processing(),
                self.swapchain_objects.as_mut().unwrap(),
                &mut self.device_context,
            );
        }

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
