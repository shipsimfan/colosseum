use crate::{
    Result,
    settings::SettingsCache,
    update::{
        UpdateContext,
        components::{Camera, CameraProjection, Transform},
    },
    warning,
};
use alexandria::math::{Matrix4x4f, Vector3f};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Execute all physics systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_physics_systems(&mut self) {
        self.ecs.execute_physics_systems(self.physics);
    }

    /// Execute all rendering systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_rendering_systems(&mut self) -> Result<()> {
        self.render_data.wait_for_copy()?;
        self.render_data.reset();

        self.render_data
            .set_render_scale(self.settings.display_settings().render_scale());
        self.render_data
            .set_gamma(self.settings.display_settings().gamma());
        self.render_data.set_exposure(*self.exposure);
        self.render_data.set_contrast(*self.contrast);
        self.render_data.set_saturation(*self.saturation);
        self.render_data
            .set_skybox(self.skybox.to_render(self.ecs, self.render_objects.cube()));
        self.render_data
            .set_anti_aliasing(self.settings.display_settings().anti_aliasing());
        self.render_data
            .lighting_mut()
            .set_shadow_quality(self.settings.display_settings().shadow_quality());

        if !self.update_camera() {
            warning!(self.logger, "no active camera set");
            self.render_data.set_camera(
                Matrix4x4f::IDENTITY,
                Vector3f::ZERO,
                CameraProjection::default(),
                Matrix4x4f::IDENTITY,
                1.0,
            );
            return Ok(());
        }
        self.render_data
            .lighting_mut()
            .set_ambient_light(*self.ambient_light);

        self.ecs.execute_rendering_systems(self.render_data);

        Ok(())
    }

    /// Update the camera in the render data, returning whether an active camera was set
    fn update_camera(&mut self) -> bool {
        // Verify there is an active camera
        let active_camera = match self.active_camera {
            &mut Some(id) => id,
            None => return false,
        };

        // Get the projection matrix from the active camera
        let (projection_matrix, projection, aspect) =
            match self.ecs.try_get_mut::<Camera>(active_camera) {
                Some(camera) => camera.projection_matrix(self.window_size),
                None => {
                    *self.active_camera = None;
                    return false;
                }
            };

        // Get the transform associated with the camera and combine it with the projection matrix
        let ((view, inverse_view), position) =
            match self.ecs.try_get_mut::<Transform>(active_camera) {
                Some(transform) => (transform.camera_matrix(), transform.position()),
                None => ((Matrix4x4f::IDENTITY, Matrix4x4f::IDENTITY), Vector3f::ZERO),
            };

        // Set the view-projection matrix in the render data
        self.render_data.set_camera(
            projection_matrix * view,
            position,
            projection,
            inverse_view,
            aspect,
        );
        true
    }
}
