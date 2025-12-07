use crate::graphics::{
    CameraHandle, GraphicsContext, MaterialHandle, MeshRendererHandle,
    lights::{DirectionalLightHandle, PointLightHandle, SpotLightHandle},
};

impl GraphicsContext {
    /// Remove `camera` from this context
    pub fn remove_camera(&mut self, camera: CameraHandle) {
        self.cameras.remove(camera);
    }

    /// Remove `material` from this context
    pub fn remove_opaque_material(&mut self, opaque_material: MaterialHandle) {
        self.opaque_materials.remove(opaque_material);
    }

    /// Remove `mesh_renderer` from this context
    pub fn remove_mesh_renderer(&mut self, mesh_renderer: MeshRendererHandle) {
        self.mesh_renderers.remove(mesh_renderer);
    }

    /// Remove `directional_light` from this context
    pub fn remove_directional_light(&mut self, directional_light: DirectionalLightHandle) {
        self.lights.remove_directional_light(directional_light);
    }

    /// Remove `point_light` from this context
    pub fn remove_point_light(&mut self, point_light: PointLightHandle) {
        self.lights.remove_point_light(point_light);
    }

    /// Remove `spot_light` from this context
    pub fn remove_spot_light(&mut self, spot_light: SpotLightHandle) {
        self.lights.remove_spot_light(spot_light);
    }
}
