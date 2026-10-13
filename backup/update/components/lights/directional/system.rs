use crate::{
    render::{RenderData, RenderDirectionalLight, ShadowMapLight},
    system_with_extra_data_and_setup,
    update::{
        components::{CameraProjection, DirectionalLight},
        ecs::System,
    },
};
use alexandria::math::{Matrix4x4f, Vector3f};

impl DirectionalLight {
    /// Create a system that operates on the [`DirectionalLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) = system_with_extra_data_and_setup!(
            |entity_count, render_data: RenderData| {
                render_data
                    .reserve_directional_lights(entity_count)
                    .unwrap();
            },
            |render_data, directional_lights: DirectionalLight| {
                let (camera_projection, inverse_view, aspect) =
                    render_data.camera_projection().clone();

                let shadow_quality = render_data.lighting().shadow_quality() as usize;
                let size = RenderDirectionalLight::SIZE[shadow_quality].x as _;

                for light in directional_lights {
                    let (view_projection_matrices, far_depths) = find_view_projection_matrices(
                        &camera_projection,
                        inverse_view,
                        aspect,
                        light.direction,
                        light.shadow_depth,
                        light.lambda,
                        size,
                    );

                    render_data.lighting_mut().add_directional_light(
                        RenderDirectionalLight {
                            color: (light.color * light.intensity).with_alpha(1.0),
                            direction: light.direction,
                            reserved: 0,
                            cascade_far_depths: far_depths,
                        },
                        view_projection_matrices,
                    );
                }
            }
        );
        System::new(type_ids, system)
    }
}

fn find_view_projection_matrices(
    camera_projection: &CameraProjection,
    inverse_view: Matrix4x4f,
    aspect: f32,

    direction: Vector3f,
    shadow_depth: f32,
    lambda: f32,

    size: f32,
) -> ([Matrix4x4f; 4], [f32; 4]) {
    let (mut camera_corners, far_depths) = camera_projection.shadow_corners(aspect, lambda);

    // Transform the shadow corners into world space
    for (plane, _) in &mut camera_corners {
        for corner in plane {
            *corner = inverse_view.transform_point(*corner);
        }
    }

    (
        [
            find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[1].0],
                direction,
                shadow_depth,
                camera_corners[1].1,
                size,
            ),
            find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[2].0],
                direction,
                shadow_depth,
                camera_corners[2].1,
                size,
            ),
            find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[3].0],
                direction,
                shadow_depth,
                camera_corners[3].1,
                size,
            ),
            find_view_projection_matrix(
                [camera_corners[0].0, camera_corners[4].0],
                direction,
                shadow_depth,
                camera_corners[4].1,
                size,
            ),
        ],
        far_depths,
    )
}

fn find_view_projection_matrix(
    camera_corners: [[Vector3f; 4]; 2],
    direction: Vector3f,
    shadow_depth: f32,
    depth: f32,
    size: f32,
) -> Matrix4x4f {
    // Compute the view matrix for the directional light
    let mut up = Vector3f::Y;
    if up.dot(direction) > 0.999 {
        up = Vector3f::X;
    }

    let view = Matrix4x4f::new_look_at(Vector3f::ZERO, direction, up);

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
    min.z -= shadow_depth;

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
