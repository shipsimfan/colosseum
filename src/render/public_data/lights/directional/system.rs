use crate::{
    SettingsCache, Window,
    render::{
        Camera, DirectionalLight, FrameIndex, GpuDirectionalLight, GpuDirectionalLightMatrix,
        HostDataBuffer,
    },
};
use alexandria::math::{Matrix4x4f, Vector3f};
use colosseum_ecs::{Res, ResMut};

impl DirectionalLight {
    /// The system to copy directional light data into the host buffer
    pub(in crate::render) fn system<Game: crate::Game>(
        frame_index: Res<FrameIndex>,
        mut light_buffer: ResMut<HostDataBuffer<GpuDirectionalLight>>,
        mut matrix_buffer: ResMut<HostDataBuffer<GpuDirectionalLightMatrix>>,
        directional_light: Option<Res<DirectionalLight>>,
        camera: Res<Camera>,
        window: Res<Window>,
        settings: Res<Game::SettingsCache>,
    ) {
        let directional_light = match directional_light {
            Some(directional_light) => directional_light,
            None => return,
        };

        let size = settings
            .display_settings()
            .shadow_quality()
            .directional_light_size();

        let window_size = window.size();
        let aspect = window_size.x as f32 / window_size.y as f32;

        let (camera_corners, far_depths) = camera
            .projection
            .shadow_corners(aspect, directional_light.shadow_lambda);
        let inverse_view = camera.inverse_view_matrix();

        light_buffer[&frame_index][0] = directional_light.gpu(far_depths);
        matrix_buffer[&frame_index][0] =
            directional_light.view_projection(camera_corners, inverse_view, size);
    }

    /// Create a new [`GpuDirectionalLight`] for this spotlight
    fn gpu(&self, far_depths: [f32; 4]) -> GpuDirectionalLight {
        GpuDirectionalLight::new(self.color.with_alpha(1.0), self.direction, far_depths)
    }

    /// Create a new [`GpuDirectionalLightMatrix`] for this spotlight
    fn view_projection(
        &self,
        mut camera_corners: [([Vector3f; 4], f32); 5],
        inverse_view: Matrix4x4f,
        size: f32,
    ) -> GpuDirectionalLightMatrix {
        // Transform the shadow corners into world space
        for (plane, _) in &mut camera_corners {
            for corner in plane {
                *corner = inverse_view.transform_point(*corner);
            }
        }

        GpuDirectionalLightMatrix::new([
            self.find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[1].0],
                camera_corners[1].1,
                size,
            ),
            self.find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[2].0],
                camera_corners[2].1,
                size,
            ),
            self.find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[3].0],
                camera_corners[3].1,
                size,
            ),
            self.find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[4].0],
                camera_corners[4].1,
                size,
            ),
        ])
    }

    fn find_view_projection_matrix(
        &self,
        camera_corners: [[Vector3f; 4]; 2],
        depth: f32,
        size: f32,
    ) -> Matrix4x4f {
        // Compute the view matrix for the directional light
        let mut up = Vector3f::Y;
        if up.dot(self.direction) > 0.999 {
            up = Vector3f::X;
        }

        let view = Matrix4x4f::new_look_at(Vector3f::ZERO, self.direction, up);

        // Convert the camera corner's into view space and find the bounds for the orthographic projection
        let mut min = Vector3f::INFINITY;
        let mut max = Vector3f::NEG_INFINITY;
        for plane in camera_corners {
            for corner in plane {
                let corner = view.transform_point(corner);

                min = min.min_v(corner);
                max = max.max_v(corner);
            }
        }

        // Add additional `shadow_depth` to the orthographic projection
        min.z -= self.shadow_depth;

        // Round the bounds based on texel size
        let texel_size: f32 = (depth * 2.0) / size;
        let half_span = texel_size * size * 0.5;

        // Snap the CENTER only
        let center = (min.xy() + max.xy()) * 0.5;
        let snapped_center = (center / texel_size).floor() * texel_size;

        // Compute the orthographic projection matrix
        let projection = Matrix4x4f::new_orthographic(
            snapped_center.x - half_span,
            snapped_center.x + half_span,
            snapped_center.y + half_span,
            snapped_center.y - half_span,
            min.z,
            max.z,
        );

        // Combine the view and projection matrices
        projection * view
    }
}
