use crate::{
    render::{RenderData, RenderDirectionalLight},
    system_with_extra_data,
    update::{components::DirectionalLight, ecs::System},
};
use alexandria::math::{Matrix4x4f, Vector3f};

impl DirectionalLight {
    /// Create a system that operates on the [`DirectionalLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) = system_with_extra_data!(
            |render_data: RenderData, directional_lights: DirectionalLight| {
                render_data
                    .reserve_directional_lights(directional_lights.len())
                    .unwrap();

                let camera_corners = render_data.camera_corners();
                let camera_depth = render_data.camera_depth();

                for light in directional_lights {
                    render_data.lighting_mut().add_directional_light(
                        RenderDirectionalLight {
                            color: (light.color * light.intensity).with_alpha(1.0),
                            direction: light.direction,
                        },
                        find_view_projection_matrix(
                            camera_corners,
                            light.direction,
                            light.shadow_depth,
                            camera_depth,
                        ),
                    );
                }
            }
        );
        System::new(type_ids, system)
    }
}

fn find_view_projection_matrix(
    camera_corners: [[Vector3f; 4]; 2],
    direction: Vector3f,
    shadow_depth: f32,
    camera_depth: f32,
) -> Matrix4x4f {
    // Compute the view matrix for the directional light
    let mut up = Vector3f::Y;
    if up.dot(direction) > 0.999 {
        up = Vector3f::X;
    }

    let view = Matrix4x4f::new_look_at(Vector3f::ZERO, direction, up).inverse();

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
    const SHADOW_MAP_SIZE: f32 = 2048.0;
    let texel_size: f32 = (camera_depth * 2.0) / SHADOW_MAP_SIZE;
    let half_span = texel_size * SHADOW_MAP_SIZE * 0.5;

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
